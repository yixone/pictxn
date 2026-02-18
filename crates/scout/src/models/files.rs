use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScoutFile {
    /// File URL in different variants
    pub files: ScoutFileUrls,

    /// Media width
    pub width: Option<usize>,
    /// Media height
    pub height: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ScoutFileUrls {
    /// URL of the file to preview
    pub preview: Option<String>,
    /// URL of the file for normal display
    pub sample: Option<String>,
    /// URL of the original file
    pub original: String,
}
