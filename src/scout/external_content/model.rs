use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::{
    domains::content_sources::id::ContentSourceId, scout::external_content::id::ExternalContentId,
};

/// Content received from external channels in the background
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct ExternalContent {
    /// Unique External Content ID
    id: ExternalContentId,
    /// Content ID from source
    external_id: String,

    /// Content loading time into the database
    created: DateTime<Utc>,

    title: Option<String>,
    description: Option<String>,

    /// Original content source ID
    source_id: ContentSourceId,

    /// Image width
    media_width: Option<u32>,
    /// Image height
    media_height: Option<u32>,

    /// Link to the preview file
    file_preview_url: Option<String>,
    /// Link to the original file
    file_url: String,
}
