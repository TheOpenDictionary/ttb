use std::{collections::HashMap, error::Error, fs::File};

use bzip2::{write::BzEncoder, Compression};
use indicatif::{ProgressBar, ProgressStyle};

use once_cell::sync::Lazy;
use serde_json::{Map, Number, Value};
use tantivy::{doc, Index};
use tar::Builder;
use tempfile::TempDir;

use super::{
    constants::ARCHIVE_NAME,
    schema::{FIELD_LANGUAGE, FIELD_LENGTH, FIELD_TEXT, FIELD_TRANSLATIONS, SCHEMA},
    tatoeba::Sentence,
};

pub fn build_index(
    sentences: impl Iterator<Item = Sentence>,
    links: HashMap<u64, Vec<u64>>,
) -> Result<(), Box<dyn Error>> {
    let tmp = TempDir::new()?;
    let progress = ProgressBar::new_spinner();
    let index = Index::create_in_dir(&tmp, SCHEMA.clone())?;

    progress.set_style(
        ProgressStyle::default_bar().template("{spinner} {human_pos} sentences indexed")?,
    );

    let mut index_writer = index.writer(100_000_000)?;

    for s in sentences {
        progress.inc(1);
        // let default = vec![];
        // let translations = links.get(&s.id).unwrap_or(&default);

        let d = doc!(
          *FIELD_TEXT => s.text.as_str(),
          *FIELD_LANGUAGE => s.language.as_str(),
          *FIELD_LENGTH => s.text.len() as u64
        );

        // let trans: Map<String, Value> = translations.iter().fold(Map::new(), |mut accum, item| {
        //     if let Some(sent) = sentences.get(item) {
        //         accum.insert(sent.language.clone(), Value::Number(Number::from(sent.id)));
        //     }
        //     accum
        // });

        // d.add_json_object(*FIELD_TRANSLATIONS, trans);

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
