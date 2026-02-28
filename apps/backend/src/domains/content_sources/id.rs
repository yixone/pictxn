use serde::Serialize;
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize)]
#[sqlx(transparent)]
pub struct ContentSourceId(pub Uuid);

impl ContentSourceId {
    pub fn generate() -> Self {
        let inner = Uuid::new_v4();
        ContentSourceId(inner)
    }
}
