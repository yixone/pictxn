use serde::Serialize;
use sqlx::FromRow;

/// Content received from external channels in the background
#[derive(Debug, Clone, PartialEq, FromRow, Serialize)]
pub struct ExternalContent {
    /// Content ID from source
    pub external_id: String,

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
