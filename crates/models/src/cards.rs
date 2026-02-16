use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, PartialEq, Type)]
#[sqlx(transparent)]
pub struct CardId(pub i64);

/// Card domain
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct Card {
    /// Unique Card id
    pub id: CardId,

    /// Card creation date
    pub created: DateTime<Utc>,

    /// Card title
    pub title: Option<String>,
    /// Card description
    pub description: Option<String>,
}
