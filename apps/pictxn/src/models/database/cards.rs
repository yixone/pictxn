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
    id: CardId,
    author_id: UserId,

    created_at: DateTime<Utc>,

    title: Option<String>,
    description: Option<String>,

    visibility: CardVisibility,
    deleted_at: Option<DateTime<Utc>>,
}

pub struct CardFilesJoined {
    card: CardDb,
    files: Vec<FileDb>,
}
