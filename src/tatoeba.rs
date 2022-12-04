use std::{error::Error, fs::File};

use bzip2::read::BzDecoder;
use csv::{ReaderBuilder, StringRecord};

use crate::{download::download_file, utils::read_temp_file};

#[derive(Deserialize)]
pub struct Sentence {
    pub id: String,
    pub language: String,
    pub text: String,
}

async fn get_resource(file_name: &str) -> Result<(), Box<dyn Error>> {
    let path = &format!(".tmp/{}", file_name);
    let url = &format!("https://downloads.tatoeba.org/exports/{}", file_name);

    download_file(url, path).await?;

    let file = File::open(path)?;
    let decoder = BzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(".tmp")?;

    Ok(())
}

pub async fn get_sentences() -> Result<impl Iterator<Item = Sentence>, Box<dyn Error>> {
    get_resource("sentences.tar.bz2").await?;

    let output = read_temp_file("sentences.csv")?;

    let mut builder = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(output);

    builder.set_headers(StringRecord::from(vec!["id", "language", "text"]));

    let data = builder.into_deserialize::<Sentence>().map(|r| r.unwrap());

    Ok(data)
}
