use std::str::FromStr;

use crate::errors::UserError;

/// Unique UUID identifier for the user
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub struct UserId(pub uuid::Uuid);

impl UserId {
    pub fn generate() -> Self {
        UserId(uuid::Uuid::new_v4())
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = uuid::Uuid::from_str(s)?;
        Ok(UserId(inner))
    }
}

const MIN_USERNAME_LEN: usize = 3;
const MAX_USERNAME_LEN: usize = 32;

#[derive(Debug, Clone, PartialEq, Hash, derive_more::Display)]
pub struct Username(pub String);

impl Username {
    pub fn try_from_raw(r: &str) -> Result<Self, UserError> {
        let trimmed = r.trim();

        if !trimmed.is_ascii() || trimmed.is_empty() || trimmed.contains(char::is_whitespace) {
            return Err(UserError::InvalidUsername);
        }

        let username = trimmed.to_lowercase();
        if !(MIN_USERNAME_LEN..=MAX_USERNAME_LEN).contains(&username.len()) {
            return Err(UserError::InvalidUsername);
        }

        Ok(Self(username))
    }
}

impl FromStr for Username {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Username::try_from_raw(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordHash(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Role {
    #[default]
    Regular,
    Admin,
}

impl Role {
    pub fn is_regular(&self) -> bool {
        matches!(self, Role::Regular)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }
}
