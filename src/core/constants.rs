use std::path::PathBuf;

use dirs::data_dir;
use once_cell::sync::Lazy;
use reqwest::Client;

pub const HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());
pub const INDEX_DIR: Lazy<PathBuf> = Lazy::new(|| data_dir().unwrap().join(".ttb").join("store"));
pub const ARCHIVE_NAME: &str = "tatoeba.tar.bz2";
