use crate::storage::types::hash::FileHash;

pub struct OutputSaveFile {
    /// Hash of the saved file
    pub file_hash: FileHash,
    /// Saved file size (in bytes)
    pub file_size: u64,
    /// File upload time
    pub timestamp: i64,
    /// Time taken to upload the file
    pub uploading_time: u64,
}
