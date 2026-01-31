use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserDomain {
    id: UserId,

    created_at: DateTime<Utc>,

    username: String,
    password_hash: String,
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn generate() -> Self {
        UserId(Uuid::new_v4())
    }
}
