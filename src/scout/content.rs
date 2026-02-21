use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScoutContentItem {
    /// The channel that created the card
    pub channel_name: &'static str,

    /// Card title
    pub title: Option<String>,
    /// Card description
    pub description: Option<String>,

    /// URL from the file was obtained
    pub origin_url: Option<String>,

    /// Media width
    pub media_width: Option<usize>,
    /// Media height
    pub media_height: Option<usize>,

    /// URL of the file to preview
    pub file_preview: Option<String>,
    /// URL of the file for normal display
    pub file_sample: Option<String>,
    /// URL of the original file
    pub file_original: String,
}
