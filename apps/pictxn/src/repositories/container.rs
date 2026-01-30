use sqlx::SqlitePool;

pub struct RepositoriesContainer {}

impl RepositoriesContainer {
    pub fn new(pool: SqlitePool) -> Self {
        RepositoriesContainer {}
    }
}
