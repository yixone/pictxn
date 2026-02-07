use chrono::{DateTime, Utc};

use crate::types::user::{PasswordHash, Role, UserId, Username};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: UserId,

    password: PasswordHash,

    username: Username,
    created: DateTime<Utc>,

    role: Role,
}

impl User {
    pub fn new(
        id: UserId,
        password: PasswordHash,
        username: Username,
        created: DateTime<Utc>,
        role: Role,
    ) -> Self {
        Self {
            id,
            password,
            username,
            created,
            role,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn password(&self) -> &PasswordHash {
        &self.password
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn role(&self) -> Role {
        self.role
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUserInput {
    pub username: Username,
    pub password: String,
}
