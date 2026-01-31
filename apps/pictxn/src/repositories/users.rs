use sqlx::SqlitePool;

pub struct UsersRepository {
    pool: SqlitePool,
}

impl UsersRepository {
    pub fn new(pool: SqlitePool) -> UsersRepository {
        UsersRepository { pool }
    }
}
