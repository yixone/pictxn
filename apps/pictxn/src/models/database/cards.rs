use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::models::{
    database::files::FileDb,
    domains::{
        cards::{CardId, CardVisibility},
        users::UserId,
    },
};

#[derive(Debug, FromRow)]
pub struct CardDb {
    pub id: CardId,
    pub author_id: UserId,

    pub created_at: DateTime<Utc>,

    pub title: Option<String>,
    pub description: Option<String>,

    pub visibility: CardVisibility,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct CardFilesJoined {
    card: CardDb,
    files: Vec<FileDb>,
}
