use std::{
    error::Error,
    fs::{create_dir, remove_dir_all, File},
};

use bzip2::{write::BzEncoder, Compression};
use indicatif::{ProgressBar, ProgressStyle};
use tantivy::{
    doc,
    schema::{Schema, STORED, STRING, TEXT},
    Index,
};
use tar::Builder;

use crate::TEMP_DIR;

use super::{tatoeba::Sentence, utils::exists};

pub fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("text", TEXT | STORED);
    schema_builder.add_text_field("language", STRING);
    schema_builder.build()
}

pub async fn build_index(sentences: Vec<Sentence>) -> Result<(), Box<dyn Error>> {
    let schema = create_schema();
    let index_path = &format!("{}/idx", TEMP_DIR);
    let progress = ProgressBar::new(sentences.len() as u64);

    progress.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );

    progress.tick();

    if !exists(index_path) {
        create_dir(index_path)?;
    }

    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let text = schema.get_field("text").unwrap();
    let language = schema.get_field("language").unwrap();

    let mut index_writer = index.writer(50_000_000).unwrap();

    for s in sentences {
        progress.inc(1);
        progress.set_message(format!("Indexing sentence #{}", s.id));
        index_writer
            .add_document(doc!(text => s.text, language => s.language))
            .unwrap();
    }

    index_writer.commit()?;

    progress.set_message("Compressing into an archive...");

    let file = File::create("tatoeba.tar.bz2")?;
    let encoder = BzEncoder::new(file, Compression::best());
    let mut builder = Builder::new(encoder);

    builder.append_dir_all(".", &index_path)?;
    builder.finish()?;

    progress.set_message("Cleaning up...");

    remove_dir_all(&index_path)?;

    progress.finish();

    Ok(())
}
