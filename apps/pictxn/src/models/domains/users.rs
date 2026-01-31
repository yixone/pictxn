use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserDomain {
    id: UserId,

    created_at: DateTime<Utc>,

    username: String,
    password_hash: String,
}

impl UserDomain {
    pub fn new(username: String, password_hash: String) -> Self {
        UserDomain {
            id: UserId::generate(),
            created_at: Utc::now(),
            username,
            password_hash,
        }
    }
}

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn generate() -> Self {
        UserId(Uuid::new_v4())
    }
}
