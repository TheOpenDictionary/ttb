use reqwest::Client;
use std::{error::Error, fs::write};

use crate::utils::file_exists;

pub async fn download_file(url: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    if !file_exists(path) {
        let body = client.get(url).send().await?.bytes().await?;
        write(path, &body)?;
    }

    Ok(())
}
