use crate::db::notes::NoteTitle;
use crate::lexical::{EditorState, GetRawText};
use crate::models::notes::Note;
use std::fs::create_dir;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::usize;
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, RegexQuery};
use tantivy::{schema::*, Index, IndexReader, IndexWriter, ReloadPolicy};

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

const TITLE: &str = "title";
const CONTENT: &str = "content";
const ID: &str = "id";

pub fn initialize(
    path: PathBuf,
) -> tantivy::Result<(Arc<TextIndexWriter>, Arc<TextIndexSearcher>, bool)> {
    let path = path.join("tantivy");

    let mut needs_reindex = false;
    match create_dir(path.clone()) {
        Ok(_) => {
            needs_reindex = true;
        }
        Err(_) => {}
    };

    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field(TITLE, TEXT | STORED);
    schema_builder.add_text_field(CONTENT, TEXT);
    schema_builder.add_text_field(ID, STRING | STORED);

    let schema = schema_builder.build();
    let directory = tantivy::directory::MmapDirectory::open(path)?;

    let index = Index::open_or_create(directory, schema.clone())?;

    let writer = index
        .writer(50_000_000)
        .expect("a writer should be able to be created");
    let writer = Mutex::new(writer);

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let searcher = TextIndexSearcher {
        index: index.clone(),
        schema: schema.clone(),
        reader: reader.clone(),
    };
    let searcher = Arc::new(searcher);

    let writer = TextIndexWriter {
        index: index.clone(),
        schema: schema.clone(),
        reader: reader.clone(),
        writer,
    };
    let writer = Arc::new(writer);

    Ok((writer, searcher, needs_reindex))
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

pub fn search_by_title(
    search: &str,
    limit: usize,
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
            searcher.search(&query, &TopDocs::with_limit(limit))?
        }
        false => {
            let query = regex::escape(search).to_lowercase();
            let query = format!(r"{query}.*");
            let query = RegexQuery::from_pattern(&query, title)?;
            searcher.search(&query, &TopDocs::with_limit(limit))?
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
