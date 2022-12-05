use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use reqwest::{Client, IntoUrl};
use std::{borrow::Cow, cmp::min, error::Error, fs::File, io::Write, path::Path};
use tokio_stream::StreamExt;

use super::utils::exists;

const CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub async fn download_file<P: AsRef<Path>>(
    url: &str,
    path: &P,
    prefix: String,
    mp: &MultiProgress,
) -> Result<(), Box<dyn Error>> {
    if exists(path) {
        return Ok(());
    }

    let res = CLIENT
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", url)))?;

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", url))?;

    let pb = mp.add(ProgressBar::new(total_size));

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{prefix}] {bytes}/{total_bytes} {wide_bar}")?,
    );

    pb.set_prefix(prefix);

    let mut file = File::create(path.as_ref()).or(Err(format!(
        "Failed to create file '{}'",
        path.as_ref().display()
    )))?;

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
