use console::Emoji;
use futures::join;
use std::error::Error;
use tempfile::TempDir;

use crate::{
    cli::stepper::Stepper,
    core::{
        indexer::build_index,
        tatoeba::{download_resource, read_sentences_from_csv, TatoebaResource},
        utils::extract_tar_bz2,
    },
};

const LIGHTNING: Emoji = Emoji("⚡", "");

pub async fn build() -> Result<(), Box<dyn Error>> {
    let mut stepper = Stepper::new(4);
    let tmp = TempDir::new()?;

    stepper.print_step("🌍", "Downloading latest Tatoeba data...");

    let sentences_file_name = download_resource(TatoebaResource::Sentences, &tmp).await?;
    let links_file_name = download_resource(TatoebaResource::Links, &tmp).await?;

    stepper.print_step("💥", "Extracting archives...");

    let _ = join!(
        extract_tar_bz2(&sentences_file_name, &tmp),
        extract_tar_bz2(&links_file_name, &tmp),
    );

    stepper.print_step("🧠", "Loading sentences into memory...");

    let sentences = read_sentences_from_csv(&tmp.path().join("sentences.csv")).await?;

    stepper.print_step("🏗️ ", "Building index...");

    build_index(sentences)?;

    println!("\n\n{} All done!", LIGHTNING);

    Ok(())
}
