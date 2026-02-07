#[derive(Debug, Clone, PartialEq, Hash, derive_more::Display)]
pub struct FileId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FileState {
    #[default]
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
