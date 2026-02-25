use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::scout::external_content::id::ExternalContentId;

/// Content received from external channels in the background
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct ExternalContent {
    /// Unique External Content ID
    pub id: ExternalContentId,
    /// Content ID from source
    pub external_id: String,

    /// Content loading time into the database
    pub created: DateTime<Utc>,

    pub title: Option<String>,
    pub description: Option<String>,

    /// Original content source
    pub source: String,

    /// Image width
    pub media_width: Option<u32>,
    /// Image height
    pub media_height: Option<u32>,

    /// Link to the preview file
    pub file_preview_url: Option<String>,
    /// Link to the original file
    pub file_url: String,
}
