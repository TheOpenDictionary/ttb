use std::{error::Error, fs::create_dir};

use tantivy::{
    schema::{Schema, STORED, TEXT},
    Index,
};

use crate::{tatoeba::Sentence, utils::file_exists, TEMP_DIR};

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("text", TEXT | STORED);
    schema_builder.add_text_field("language", TEXT);
    schema_builder.build()
}

pub fn build_index(sentences: impl Iterator<Item = Sentence>) -> Result<(), Box<dyn Error>> {
    let schema = create_schema();
    let index_path = &format!("{}/idx", TEMP_DIR);
    println!("{}", index_path);
    if !file_exists(index_path) {
        create_dir(index_path)?;
    }

    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let sentence = schema.get_field("text").unwrap();
    let language = schema.get_field("language").unwrap();

    let mut index_writer = index.writer(50_000_000).unwrap();

    sentences.for_each(|s| {
        index_writer
            .add_document(doc!(
              sentence => s.text,
              language => s.language
            ))
            .unwrap();
    });

    index_writer.commit()?;

    Ok(())
}
