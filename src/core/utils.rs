use std::{
    error::Error,
    fs::{metadata, File},
    path::Path,
};

use bzip2::read::BzDecoder;

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    metadata(path).is_ok()
}

pub fn read_temp_file(file_name: &str) -> Result<File, std::io::Error> {
    File::open(format!(".tmp/{}", file_name))
}

pub fn extract_tar_bz2(path: &str, dst: &Path) -> Result<(), Box<dyn Error>> {
    println!("{}", path);
    let file = File::open(path)?;
    let decoder = BzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(&dst)?;

    Ok(())
}
