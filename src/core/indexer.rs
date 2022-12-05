use std::{error::Error, fs::File};

use bzip2::{write::BzEncoder, Compression};
use indicatif::{ProgressBar, ProgressStyle};

use tantivy::{doc, Index};
use tar::Builder;
use tempfile::TempDir;

use super::{
    schema::{FIELD_LANGUAGE, FIELD_TEXT, SCHEMA},
    tatoeba::Sentence,
};

pub fn build_index(sentences: Vec<Sentence>) -> Result<(), Box<dyn Error>> {
    let tmp = TempDir::new()?;
    let progress = ProgressBar::new(sentences.len() as u64);
    let index = Index::create_in_dir(&tmp, SCHEMA.clone())?;

    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {human_pos}/{human_len} sentences {wide_bar}")?,
    );

    let mut index_writer = index.writer(50_000_000)?;

    for s in sentences {
        progress.inc(1);
        index_writer.add_document(doc!(*FIELD_TEXT => s.text, *FIELD_LANGUAGE => s.language))?;
    }

    progress.finish();

    index_writer.commit()?;
    index_writer.wait_merging_threads()?;

    let file = File::create("tatoeba.tar.bz2").unwrap();
    let encoder = BzEncoder::new(file, Compression::best());
    let mut builder = Builder::new(encoder);

    builder.append_dir_all(".", &tmp)?;

    builder.finish().unwrap();

    Ok(())
}
