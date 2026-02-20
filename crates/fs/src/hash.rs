use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileHash {
    Sha256([u8; 32]),
}

impl FileHash {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            FileHash::Sha256(b) => b,
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        match self {
            FileHash::Sha256(b) => b.to_vec(),
        }
    }

    pub fn to_hex(self) -> String {
        let buff = self.to_vec();
        hex::encode(buff)
    }
}

impl From<Sha256> for FileHash {
    fn from(value: Sha256) -> Self {
        FileHash::Sha256(value.finalize().into())
    }
}
