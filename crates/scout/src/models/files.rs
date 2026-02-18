use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExternalFile {
    pub id: String,
    pub files: ExternalFileUrls,

    pub hash: Option<String>,

    pub width: Option<usize>,
    pub height: Option<usize>,

    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct ExternalFileUrls {
    pub preview: String,
    pub sample: String,
    pub source: String,
}
