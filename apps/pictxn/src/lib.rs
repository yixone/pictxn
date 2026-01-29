pub mod database;
pub mod helpers;
pub mod storage;
pub mod streams;
pub mod types;

pub(crate) const MEGABYTES: usize = 1024 * 1024;

pub(crate) const APP_NAME: &str = "pictxn";
pub(crate) const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) const DATA_DIR: &str = "/data";
pub(crate) const MEDIA_DIR: &str = "/media";
pub(crate) const TEMP_DIR: &str = "/temp";
pub(crate) const DB_FILE: &str = "/pictxn.db";
