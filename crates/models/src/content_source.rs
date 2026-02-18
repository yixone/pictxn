use sqlx::{Type, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Type)]
#[sqlx(transparent)]
pub struct SourceId(pub Uuid);

/// Information about the content source
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct ContentSource {
    /// Unique Content Source id
    pub id: SourceId,

    /// Content Source Domain
    pub source_domain: String,
}
