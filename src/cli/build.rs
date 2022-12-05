use std::error::Error;

use console::{style, Emoji};
use tempfile::TempDir;
use tokio::fs::create_dir_all;

use crate::{
    cli::stepper::Stepper,
    core::{
        indexer::build_index,
        tatoeba::{download_resource, read_sentences_from_csv, TatoebaResource},
        utils::extract_tar_bz2,
    },
    TEMP_DIR,
};

pub async fn build() -> Result<(), Box<dyn Error>> {
    let mut stepper = Stepper::new(4);
    let tmp = TempDir::new()?;

    create_dir_all(TEMP_DIR)
        .await
        .unwrap_or_else(|err| println!("Couldn't create temp directory: {}", err.to_string()));

    stepper.print_step("📦", "Downloading latest Tatoeba files...");

    let file_name = download_resource(TatoebaResource::Sentences, &tmp.path()).await?;

    stepper.print_step("💥", "Extracting archive...");

    extract_tar_bz2(&file_name, tmp.path())?;

    let sentences = read_sentences_from_csv("sentences.csv").await?;

    stepper.print_step("🛠️", "Building index...");

    build_index(sentences).await.unwrap();

    Ok(())
}
