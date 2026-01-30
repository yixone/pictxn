use std::path::Path;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn open_file(file: impl AsRef<Path>) -> sqlx::Result<Self> {
        let path = file.as_ref();
        crate::helpers::fs::create_all_parents(path).await?;

        let opts = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new().connect_with(opts).await?;

        Ok(Database { pool })
    }

    pub async fn open_in_memory() -> sqlx::Result<Self> {
        let opts = SqliteConnectOptions::new()
            .in_memory(true)
            .shared_cache(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .min_connections(1)
            .connect_with(opts)
            .await?;

        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        sqlx::migrate!("../../migrations")
            .run(&self.pool)
            .await
            .map_err(sqlx::Error::from)
    }

    pub async fn close(self) {
        self.pool.close().await
    }
}
