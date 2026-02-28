use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct ScoutCard {
    pub source: Option<String>,
    pub preview_url: Option<String>,
    pub file_url: String,
}
