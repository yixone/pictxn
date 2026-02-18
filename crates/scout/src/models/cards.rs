use serde::Serialize;

use crate::{models::files::ScoutFile, providers::ProviderType};

#[derive(Debug, Serialize)]
pub struct ScoutCard {
    pub provider: ProviderType,

    pub title: Option<String>,
    pub description: Option<String>,

    pub file: ScoutFile,

    pub origin_url: String,
}
