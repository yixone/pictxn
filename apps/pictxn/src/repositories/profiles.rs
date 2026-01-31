use sqlx::SqlitePool;

pub struct ProfilesRepository {
    pool: SqlitePool,
}

impl ProfilesRepository {
    pub fn new(pool: SqlitePool) -> ProfilesRepository {
        ProfilesRepository { pool }
    }
}
