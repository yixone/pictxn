use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domains::{cards::id::CardId, files::id::FileId};

/// Card domain
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct Card {
    /// Unique Card id
    pub id: CardId,

    /// Related File ID
    pub file_id: FileId,

    /// Card creation date
    pub created: DateTime<Utc>,

    /// Card title
    pub title: Option<String>,
    /// Card description
    pub description: Option<String>,
}
