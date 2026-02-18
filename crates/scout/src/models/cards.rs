use serde::Serialize;

use crate::{models::files::ScoutFile, providers::ProviderType};

#[derive(Debug, Serialize)]
pub struct ScoutCard {
    /// The provider that created the card
    pub provider: ProviderType,

    /// Card title
    pub title: Option<String>,
    /// Card description
    pub description: Option<String>,

    /// Information about the file associated with the card
    pub file: ScoutFile,

    /// URL from the file was obtained
    pub origin_url: String,
}
