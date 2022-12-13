use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

use csv::{ReaderBuilder, StringRecord};
use indicatif::MultiProgress;
use once_cell::sync::Lazy;
use serde::Deserialize;
use strum::Display;

use super::download::download_file;

#[derive(Deserialize, Debug, Clone)]
pub struct Sentence {
    pub id: u64,
    pub language: String,
    pub text: String,
}

#[derive(Display, Debug, Eq, Hash, PartialEq)]
pub enum TatoebaResource {
    #[strum(serialize = "sentences.tar.bz2")]
    Sentences,
    #[strum(serialize = "links.tar.bz2")]
    Links,
}

const TATOEBA_FILES: Lazy<HashMap<TatoebaResource, &str>> = Lazy::new(|| {
    HashMap::from([
        (TatoebaResource::Sentences, "sentences.csv"),
        (TatoebaResource::Links, "links.csv"),
    ])
});

pub async fn download_resource<P: AsRef<Path>>(
    resource: TatoebaResource,
    dst: P,
    mp: &MultiProgress,
) -> Result<PathBuf, Box<dyn Error>> {
    let url = &format!("https://downloads.tatoeba.org/exports/{}", resource);
    let path = dst.as_ref().join(resource.to_string());

    download_file(url, &path, resource.to_string(), mp).await?;

    Ok(path)
}

pub fn get_resource_file_name(resource: TatoebaResource) -> &'static str {
    TATOEBA_FILES.get(&resource).unwrap()
}

pub async fn read_sentences_from_csv(
    csv_file: &Path,
) -> Result<impl Iterator<Item = Sentence>, Box<dyn Error>> {
    let output = File::open(csv_file)?;

    let mut builder = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(output);

    builder.set_headers(StringRecord::from(vec!["id", "language", "text"]));

    let data = builder
        .into_deserialize::<Sentence>()
        .filter_map(|r| r.ok());

    Ok(data)
}

pub async fn read_links_from_csv(
    csv_file: &Path,
) -> Result<HashMap<u64, Vec<u64>>, Box<dyn Error>> {
    let output = File::open(csv_file)?;

    let builder = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(output);

    let data: HashMap<u64, Vec<u64>> =
        builder
            .into_records()
            .filter_map(|r| r.ok())
            .fold(HashMap::new(), |mut map, r| {
                let k: u64 = r.get(0).unwrap().to_string().parse().unwrap();
                let v: u64 = r.get(1).unwrap().to_string().parse().unwrap();

                if let Some(values) = map.get_mut(&k) {
                    values.push(v);
                } else {
                    map.insert(k, vec![v]);
                }

                map
            });

    Ok(data)
}
