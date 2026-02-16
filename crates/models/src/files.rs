use chrono::{DateTime, Utc};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::content_source::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Type)]
#[sqlx(transparent)]
pub struct FileId(pub Uuid);

/// File domain
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct File {
    /// Unique file id
    pub id: FileId,

    /// Content source ID
    pub source: Option<SourceId>,
    /// URL from which the content was obtained
    pub source_url: Option<String>,

    /// File creation date
    pub created: DateTime<Utc>,

    /// Original filename
    pub filename: Option<String>,
    /// Mimetype of this file
    pub content_type: String,
    /// Size of file (in bytes)
    pub size: isize,
    /// Accent color of the file
    pub color: String,
}
