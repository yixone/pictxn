use sqlx::FromRow;

use crate::domains::content_sources::id::ContentSourceId;

/// Information about the content source
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct ContentSource {
    /// Unique Content Source id
    pub id: ContentSourceId,

    /// Content Source Domain
    pub source_domain: String,
}
