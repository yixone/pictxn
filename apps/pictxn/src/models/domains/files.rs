use uuid::Uuid;

pub struct FileDomain {
    //
}

#[derive(Debug, sqlx::Type, PartialEq, PartialOrd, Clone)]
#[sqlx(transparent)]
pub struct FileId(String);

impl FileId {
    pub fn generate() -> Self {
        FileId(Uuid::new_v4().simple().to_string())
    }
}

#[derive(Debug, sqlx::Type, Clone, Copy)]
#[sqlx(rename_all = "lowercase")]
pub enum FileState {
    Pending,
    Processing,
    Ready,
    Failed,
}

impl FileState {
    pub fn is_pending(&self) -> bool {
        matches!(self, FileState::Pending)
    }

    pub fn is_processing(&self) -> bool {
        matches!(self, FileState::Processing)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, FileState::Ready)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, FileState::Failed)
    }
}
