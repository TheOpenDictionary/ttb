use std::error::Error;

use crate::core::{constants::INDEX_DIR, utils::extract_tar_bz2};

// TODO: fill this in
pub async fn pull() -> Result<(), Box<dyn Error>> {
    extract_tar_bz2("tatoeba.tar.bz2", INDEX_DIR.as_path()).await?;

    Ok(())
}
