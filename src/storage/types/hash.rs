use sha2::Digest;

/// The SHA-256 hash calculated when saving the file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileHash(pub [u8; 32]);

impl From<sha2::Sha256> for FileHash {
    fn from(value: sha2::Sha256) -> Self {
        FileHash(value.finalize().into())
    }
}
