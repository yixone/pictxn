use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct DomainFileHash(pub Vec<u8>);
