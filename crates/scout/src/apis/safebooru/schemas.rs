use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SafebooruContentItem {
    pub preview_url: String,
    pub sample_url: String,
    pub file_url: String,
    pub hash: String,
    pub width: usize,
    pub height: usize,
    pub id: usize,
    pub image: String,
    pub owner: String,
    pub source: Option<String>,
}
