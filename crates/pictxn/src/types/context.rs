use crate::types::{
    tokens::AuthToken,
    user::{Role, UserId},
};

pub struct AuthContext {
    pub creator: AuthToken,

    pub user: UserId,
    pub role: Role,
}
