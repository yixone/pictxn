use std::fmt::Display;

use sha2::Digest;

/// Information about the file that was saved by the current function
#[derive(Debug)]
pub struct FileWriteResult {
    pub size: usize,
    pub sha256: Sha256Hash,
}

#[derive(Debug, sqlx::Type, PartialEq, PartialOrd, Clone, Copy)]
#[sqlx(transparent)]
pub struct Sha256Hash([u8; 32]);

impl Sha256Hash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Sha256Hash(bytes)
    }

    pub fn from_hasher(hasher: sha2::Sha256) -> Self {
        let bytes = hasher.finalize();
        Sha256Hash(bytes.into())
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl Display for Sha256Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
