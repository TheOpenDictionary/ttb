use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

use csv::{ReaderBuilder, StringRecord};
use serde::Deserialize;
use strum::Display;

use super::download::download_file;

#[derive(Deserialize)]
pub struct Sentence {
    pub id: String,
    pub language: String,
    pub text: String,
}

#[derive(Display, Debug)]
pub enum TatoebaResource {
    #[strum(serialize = "sentences.tar.bz2")]
    Sentences,
}

pub async fn download_resource<P: AsRef<Path>>(
    resource: TatoebaResource,
    dst: P,
) -> Result<PathBuf, Box<dyn Error>> {
    let url = &format!("https://downloads.tatoeba.org/exports/{}", resource);
    let path = dst.as_ref().join(resource.to_string());

    download_file(url, &path).await?;

    Ok(path)
}

pub async fn read_sentences_from_csv(csv_file: &Path) -> Result<Vec<Sentence>, Box<dyn Error>> {
    let output = File::open(csv_file)?;

    let mut builder = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(output);

    builder.set_headers(StringRecord::from(vec!["id", "language", "text"]));

    let data = builder
        .into_deserialize::<Sentence>()
        .map(|r| r.unwrap())
        .collect();

    Ok(data)
}
