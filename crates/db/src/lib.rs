pub mod ops;
mod repos;

use std::path::Path;

use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

// TODO: Add metrics collection

#[derive(Clone)]
pub struct Database {
    /// Database connections pool
    pool: SqlitePool,
}

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

impl Database {
    /// Open database from file
    pub async fn open_file(path: &Path) -> Result<Self, sqlx::Error> {
        if !path.exists()
            && let Some(parent) = path.parent()
        {
            std::fs::create_dir_all(parent)?;
        }

        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(path);

        let pool = SqlitePoolOptions::new().connect_with(options).await?;

        let this = Database { pool };
        Ok(this)
    }

    /// Apply migrations to the database
    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        MIGRATOR.run(&self.pool).await.map_err(sqlx::Error::from)
    }
}
