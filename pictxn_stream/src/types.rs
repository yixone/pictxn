use std::path::{Path, PathBuf};

pub struct StreamToFileResult {
    bytes_transfered: usize,
    destination: PathBuf,
    sha256: String,
}

impl StreamToFileResult {
    pub fn new(bytes_transfered: usize, destination: PathBuf, sha256: String) -> Self {
        Self {
            bytes_transfered,
            destination,
            sha256,
        }
    }

    pub fn bytes_transfered(&self) -> usize {
        self.bytes_transfered
    }

    pub fn destination(&self) -> &Path {
        &self.destination
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}
