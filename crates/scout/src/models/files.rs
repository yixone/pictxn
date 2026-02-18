use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScoutFile {
    pub files: ScoutFileUrls,

    pub width: Option<usize>,
    pub height: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ScoutFileUrls {
    pub preview: Option<String>,
    pub sample: Option<String>,
    pub source: String,
}
