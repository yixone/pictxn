#[derive(Debug, Clone, Copy, PartialEq)]
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
