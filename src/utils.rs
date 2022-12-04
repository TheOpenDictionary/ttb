use std::fs::{metadata, File};

pub fn file_exists(file_name: &str) -> bool {
    metadata(file_name).is_ok()
}

pub fn read_temp_file(file_name: &str) -> Result<File, std::io::Error> {
    File::open(format!(".tmp/{}", file_name))
}
