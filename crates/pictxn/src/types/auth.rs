use crate::types::user::{Role, UserId};

#[derive(Debug, Clone, PartialEq, Hash, derive_more::Display)]
pub struct AuthToken(pub String);

pub struct AuthContext {
    pub creator: AuthToken,

    pub user: UserId,
    pub role: Role,
}
