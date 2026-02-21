use serde::Serialize;
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct FileId(pub Uuid);

impl FileId {
    pub fn generate() -> Self {
        let inner = Uuid::new_v4();
        FileId(inner)
    }
}
