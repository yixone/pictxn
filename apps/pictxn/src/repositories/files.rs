use sqlx::SqlitePool;

pub struct FilesRepository {
    pool: SqlitePool,
}

impl FilesRepository {
    pub fn new(pool: SqlitePool) -> FilesRepository {
        FilesRepository { pool }
    }
}
