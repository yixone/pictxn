use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, Type};

use crate::files::{FileApi, FileId};

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct CardId(pub i64);

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

#[derive(Debug, Serialize)]
pub struct CardApi {
    /// Unique Card id
    pub id: CardId,
    /// Card creation date
    pub created: DateTime<Utc>,

    /// Related File
    pub file: FileApi,

    /// Card title
    pub title: Option<String>,
    /// Card description
    pub description: Option<String>,
}

impl CardApi {
    pub fn from_domains(card: Card, file: FileApi) -> Self {
        CardApi {
            id: card.id,
            created: card.created,
            file,
            title: card.title,
            description: card.description,
        }
    }
}
