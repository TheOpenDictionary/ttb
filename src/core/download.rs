use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{cmp::min, error::Error, fs::File, io::Write, path::Path};

use super::utils::exists;

const client: Lazy<Client> = Lazy::new(|| Client::new());

pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    if exists(path) {
        return Ok(());
    }

    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);

    pb.set_style(
        ProgressStyle::default_bar().template("{spinner} {bytes}/{total_bytes} {wide_bar}")?,
    );

    let mut file =
        File::create(path).or(Err(format!("Failed to create file '{}'", path.display())))?;

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;

        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;

        let new = min(downloaded + (chunk.len() as u64), total_size);

        downloaded = new;

        pb.set_position(new);
    }

    pb.finish();

    return Ok(());
}
