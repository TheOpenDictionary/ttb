use reqwest::Client;
use std::{error::Error, fs::write, path::Path};

use super::utils::exists;

pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    if !exists(path) {
        let body = client.get(url).send().await?.bytes().await?;
        write(path, &body)?;
    }

    Ok(())
}
