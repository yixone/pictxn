use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domains::{
    content_sources::id::ContentSourceId,
    files::{hash::FileHash, id::FileId},
};

/// File domain
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct File {
    /// Unique file id
    pub id: FileId,

    /// Content source ID
    pub source_id: Option<ContentSourceId>,
    /// URL from which the file was obtained
    pub source_url: Option<String>,

    /// File creation date
    pub created: DateTime<Utc>,

    /// Unique Sha256 file hash
    pub sha256: FileHash,

    /// Original filename
    pub filename: Option<String>,
    /// Mimetype of this file
    pub content_type: String,
    /// Size of file (in bytes)
    pub size: i64,
}
