use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct FileHash(pub Vec<u8>);
