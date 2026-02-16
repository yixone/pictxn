use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FileId(pub Uuid);

/// File domain
#[derive(Debug, Clone)]
pub struct File {
    /// Unique file id
    pub id: FileId,

    /// File creation date
    pub created: DateTime<Utc>,

    /// Original filename
    pub filename: String,
    /// Mimetype of this file
    pub content_type: String,
    /// Size of file (in bytes)
    pub size: isize,
    /// Accent color of the file
    pub color: String,
}
