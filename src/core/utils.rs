use std::{error::Error, fs::metadata, path::Path};

use bzip2::read::BzDecoder;
use tokio::fs::File;

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    metadata(path).is_ok()
}

pub async fn extract_tar_bz2<P: AsRef<Path>, C: AsRef<Path>>(
    path: P,
    dst: C,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(path).await?.into_std().await;
    let decoder = BzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(&dst)?;

    Ok(())
}
