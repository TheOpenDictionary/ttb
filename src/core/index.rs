use std::{collections::HashMap, error::Error, fs::File};

use bzip2::{write::BzEncoder, Compression};
use indicatif::{ProgressBar, ProgressStyle};

use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use tantivy::{doc, Index};
use tar::Builder;
use tempfile::TempDir;

use super::{
    constants::ARCHIVE_NAME,
    schema::{FIELD_LANGUAGE, FIELD_LENGTH, FIELD_TEXT, FIELD_TRANSLATIONS, SCHEMA},
    tatoeba::Sentence,
};

pub fn build_index(
    sentences: HashMap<String, Sentence>,
    links: HashMap<String, Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let tmp = TempDir::new()?;
    let progress = ProgressBar::new(sentences.len() as u64);
    let index = Index::create_in_dir(&tmp, SCHEMA.clone())?;

    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {human_pos}/{human_len} sentences {wide_bar}")?,
    );

    let mut index_writer = index.writer(100_000_000)?;

    for (_, s) in sentences.iter() {
        progress.inc(1);

        let default = vec![];
        let translations = links.get(&s.id).unwrap_or(&default);

        let mut d = doc!(
          *FIELD_TEXT => s.text.as_str(),
          *FIELD_LANGUAGE => s.language.as_str(),
          *FIELD_LENGTH => s.text.len() as u64
        );

        let trans: Map<String, Value> = translations.iter().fold(Map::new(), |mut accum, item| {
            if let Some(sent) = sentences.get(item) {
                accum.insert(item.to_string(), Value::String(sent.text.clone()));
            }
            accum
        });

        d.add_json_object(*FIELD_TRANSLATIONS, trans);

        index_writer.add_document(d)?;
    }

    progress.finish();

    index_writer.commit()?;
    index_writer.wait_merging_threads()?;

    let file = File::create(ARCHIVE_NAME).unwrap();
    let encoder = BzEncoder::new(file, Compression::best());
    let mut builder = Builder::new(encoder);

    builder.append_dir_all(".", &tmp)?;

    builder.finish().unwrap();

    Ok(())
}
