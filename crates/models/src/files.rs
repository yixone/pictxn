use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::content_source::{ContentSourceApi, SourceId};

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct FileId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct FileSha256(pub Vec<u8>);

/// File domain
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct File {
    /// Unique file id
    pub id: FileId,

    /// Content source ID
    pub source_id: Option<SourceId>,

    /// URL from which the content was obtained
    ///
    /// If `None`, the content was uploaded manually by the user.
    pub source_url: Option<String>,

    /// File creation date
    pub created: DateTime<Utc>,

    /// Sha256 file hash
    pub sha256: FileSha256,

    /// Original filename
    pub filename: Option<String>,
    /// Mimetype of this file
    pub content_type: String,
    /// Size of file (in bytes)
    pub size: i64,
}

#[derive(Debug, Serialize)]
pub struct FileApi {
    /// Unique file id
    pub id: FileId,
    /// File creation date
    pub created: DateTime<Utc>,

    /// Content source ID
    pub source: Option<ContentSourceApi>,
    /// URL from which the content was obtained
    pub source_url: Option<String>,

    /// Original filename
    pub filename: Option<String>,
    /// Mimetype of this file
    pub content_type: String,
    /// Size of file (in bytes)
    pub size: i64,
}

impl FileApi {
    pub fn from_domains(file: File, source: Option<ContentSourceApi>) -> Self {
        FileApi {
            id: file.id,
            created: file.created,
            source,
            source_url: file.source_url,
            filename: file.filename,
            content_type: file.content_type,
            size: file.size,
        }
    }
}
