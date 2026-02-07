/// Unique UUID identifier for the user
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub struct UserId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Hash, derive_more::Display)]
pub struct Username(pub String);

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
