use chrono::{DateTime, Utc};

use crate::models::domains::users::UserId;

#[derive(Debug)]
pub struct CardDomain {
    id: CardId,
    author_id: UserId,

    created_at: DateTime<Utc>,

    title: Option<String>,
    description: Option<String>,

    visibility: CardVisibility,

    deleted_at: Option<DateTime<Utc>>,
}

impl CardDomain {
    pub fn new(
        author_id: UserId,
        title: Option<String>,
        description: Option<String>,
        visibility: CardVisibility,
    ) -> Self {
        let id = CardId::generate();
        let created_at = Utc::now();
        let deleted_at = None;

        CardDomain {
            id,
            author_id,
            created_at,
            title,
            description,
            visibility,
            deleted_at,
        }
    }

    pub fn is_public(&self) -> bool {
        self.visibility.is_public()
    }

    pub fn can_be_viewed(&self, client: Option<UserId>) -> bool {
        if self.is_public() {
            true
        } else {
            client.map(|id| id == self.author_id).unwrap_or(false)
        }
    }

    pub fn is_deleted(&self) -> bool {
        match self.deleted_at {
            Some(d) => Utc::now() >= d,
            None => false,
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct CardId(i64);

impl CardId {
    pub fn generate() -> Self {
        todo!()
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum CardVisibility {
    Public,
    Private,
}

impl CardVisibility {
    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public)
    }
    pub fn is_private(&self) -> bool {
        matches!(self, Self::Private)
    }
}
