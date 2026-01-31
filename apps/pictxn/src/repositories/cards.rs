use sqlx::SqlitePool;

pub struct CardsRepository {
    pool: SqlitePool,
}

impl CardsRepository {
    pub fn new(pool: SqlitePool) -> CardsRepository {
        CardsRepository { pool }
    }
}
