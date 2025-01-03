use crate::lexical::{EditorState, GetRawText};
use crate::models::notes::{Note, NoteTitle};
use std::fs::create_dir;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, RegexQuery};
use tantivy::{
    schema::*, Index, IndexReader, IndexSettings, IndexWriter, ReloadPolicy, SnippetGenerator,
};
use tokio::sync::oneshot::Sender;

pub struct TextIndexSearcher {
    pub schema: Schema,
    pub index: Index,
    pub reader: IndexReader,
}

pub struct TextIndexWriter {
    pub reader: IndexReader,
    pub schema: Schema,
    pub index: Index,
    pub writer: Mutex<IndexWriter>,
}

pub const TITLE: &str = "title";
pub const CONTENT: &str = "content";
pub const ID: &str = "id";

pub async fn initialize(
    path: &PathBuf,
    writer_sender: Sender<Arc<TextIndexWriter>>,
    searcher_sender: Sender<Arc<TextIndexSearcher>>,
    needs_reindex: Sender<bool>,
) {
    let mut needs_reindex = Some(needs_reindex);
    let path = path.join("tantivy");
    let path_str = path
        .clone()
        .into_os_string()
        .into_string()
        .expect("Tantivy index location should be valid");
    println!("Initializing tantivy index as {path_str}");

    match create_dir(path.clone()) {
        Ok(_) => {
            if let Some(tx) = needs_reindex.take() {
                println!("No existing tantivy index, directory created");
                let _ = tx.send(true);
            }
        }
        Err(_) => {}
    };

    let mut schema_builder = Schema::builder();

    println!("Creating tantivy schema");
    let text_options = TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("en_stem")
                .set_index_option(IndexRecordOption::Basic),
        )
        .set_stored();

    schema_builder.add_text_field(TITLE, TEXT | STORED);
    schema_builder.add_text_field(CONTENT, text_options);
    schema_builder.add_text_field(ID, STRING | STORED);

    let schema = schema_builder.build();
    let directory = tantivy::directory::MmapDirectory::open(path)
        .expect("The tantivy directory should be valid");

    let index = match Index::open_or_create(directory.clone(), schema.clone()) {
        Ok(index) => {
            if let Some(tx) = needs_reindex.take() {
                println!("Schema matches detected schema");
                let _ = tx.send(false);
            }
            index
        }
        Err(_err) => {
            if let Some(tx) = needs_reindex.take() {
                println!("Tantivy index does not match existing schema, reindex needed");
                let _ = tx.send(true);
            }
            Index::create(directory, schema.clone(), IndexSettings::default())
                .expect("Tantivy should be able to create a schema")
        }
    };

    println!("Creating tantivy reader");
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .expect("Tantivy should be able to create a reader");

    let searcher = TextIndexSearcher {
        index: index.clone(),
        schema: schema.clone(),
        reader: reader.clone(),
    };
    let searcher = Arc::new(searcher);
    let _ = searcher_sender.send(searcher);

    println!("Creating tantivy writer");
    let writer = index
        .writer(50_000_000)
        .expect("a writer should be able to be created");
    let writer = Mutex::new(writer);

    let writer = TextIndexWriter {
        index: index.clone(),
        schema: schema.clone(),
        reader: reader.clone(),
        writer,
    };
    let writer = Arc::new(writer);
    let _ = writer_sender.send(writer);
}

pub fn write_note(note: Note, writer: Arc<TextIndexWriter>) -> tantivy::Result<()> {
    let schema = writer.schema.clone();
    let reader = writer.reader.clone();
    let mut writer = writer.writer.lock().unwrap();

    let title = schema.get_field(TITLE).unwrap();
    let id = schema.get_field(ID).unwrap();
    let content = schema.get_field(CONTENT).unwrap();

    let uuid = &note.uuid.to_string();

    let uuid_term = Term::from_field_text(id, uuid);
    writer.delete_term(uuid_term.clone());

    let mut doc = TantivyDocument::default();

    let body = note.body;
    let body: EditorState = serde_json::from_str(&body).expect("Foo");
    let body = match body.get_raw_text() {
        Some(body) => body,
        None => "".to_string(),
    };

    doc.add_text(title, note.title);
    doc.add_text(id, uuid);
    doc.add_text(content, body);

    writer.add_document(doc)?;

    writer.commit()?;
    reader.reload()?;

    Ok(())
}

impl NoteTitle {
    fn from_tantivy(id: &str, title: &str) -> Self {
        Self {
            uuid: id.try_into().unwrap(),
            title: title.to_string(),
        }
    }
}

use serde::Serialize;
#[derive(Serialize, Debug, PartialEq, Type)]
pub struct TitleWithContext {
    title: NoteTitle,
    context: String,
}

pub fn search_by_content(
    search: &str,
    limit: u16,
    snippet_size: u16,
    searcher: Arc<TextIndexSearcher>,
) -> tantivy::Result<Vec<TitleWithContext>> {
    let index = searcher.index.clone();
    let schema = searcher.schema.clone();
    let searcher = searcher.reader.searcher();

    let title = schema.get_field(TITLE).unwrap();
    let content = schema.get_field(CONTENT).unwrap();
    let id = schema.get_field(ID).unwrap();

    let query_parser = QueryParser::for_index(&index, vec![title, content]);
    let query = query_parser.parse_query(search)?;

    let top_results = searcher.search(&query, &TopDocs::with_limit(limit.into()))?;

    let mut snippet_generator = SnippetGenerator::create(&searcher, &*query, content)?;
    snippet_generator.set_max_num_chars(snippet_size.into());

    let all_titles: tantivy::Result<Vec<TitleWithContext>> = top_results
        .into_iter()
        .map(
            |(_score, doc_address)| -> tantivy::Result<TitleWithContext> {
                let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
                let note_id = retrieved_doc.get_first(id).unwrap();
                let note_id = note_id.as_str().unwrap();

                let note_title = retrieved_doc.get_first(title).unwrap();
                let note_title = note_title.as_str().unwrap();

                println!("{note_title}");

                let note_title = NoteTitle::from_tantivy(note_id, note_title);

                let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);
                let context = snippet.to_html();

                Ok(TitleWithContext {
                    title: note_title,
                    context,
                })
            },
        )
        .collect();

    Ok(all_titles?)
}

pub fn search_by_title(
    search: &str,
    limit: u16,
    searcher: Arc<TextIndexSearcher>,
) -> tantivy::Result<Vec<NoteTitle>> {
    let index = searcher.index.clone();
    let schema = searcher.schema.clone();
    let searcher = searcher.reader.searcher();

    let title = schema.get_field(TITLE).unwrap();
    let id = schema.get_field(ID).unwrap();

    let top_results = match search.contains(" ") {
        true => {
            let query = format!("\"{search}\"*");
            let query_parser = QueryParser::for_index(&index, vec![title]);
            let query = query_parser.parse_query(&query)?;
            searcher.search(&query, &TopDocs::with_limit(limit.into()))?
        }
        false => {
            let query = regex::escape(search).to_lowercase();
            let query = format!(r"{query}.*");
            let query = RegexQuery::from_pattern(&query, title)?;
            searcher.search(&query, &TopDocs::with_limit(limit.into()))?
        }
    };

    let all_titles: tantivy::Result<Vec<NoteTitle>> = top_results
        .into_iter()
        .map(|(_score, doc_address)| -> tantivy::Result<NoteTitle> {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            let note_id = retrieved_doc.get_first(id).unwrap();
            let note_id = note_id.as_str().unwrap();

            let note_title = retrieved_doc.get_first(title).unwrap();
            let note_title = note_title.as_str().unwrap();

            Ok(NoteTitle::from_tantivy(note_id, note_title))
        })
        .collect();

    Ok(all_titles?)
}
