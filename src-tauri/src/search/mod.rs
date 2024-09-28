use std::path::PathBuf;
use tantivy::schema::*;
use tantivy::Index;

pub fn initialize(path: PathBuf, needsReindex: bool) -> tantivy::Result<(Index, bool)> {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("id", TEXT | STORED);

    let schema = schema_builder.build();
    let directory = tantivy::directory::MmapDirectory::open(path)?;

    let index = Index::open_or_create(directory, schema)?;

    Ok((index, needsReindex))
}
