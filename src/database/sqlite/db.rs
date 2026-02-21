use std::path::Path;

use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::result::Result;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct SqliteDatabase {
    pub(super) pool: SqlitePool,
}

impl SqliteDatabase {
    /// Open database from file
    pub async fn open_file(path: &Path) -> Result<Self> {
        if !path.exists()
            && let Some(parent) = path.parent()
        {
            std::fs::create_dir_all(parent)?;
        }

        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(path);

        let pool = SqlitePoolOptions::new().connect_with(options).await?;
        Ok(SqliteDatabase { pool })
    }

    /// Apply migrations to the database
    pub async fn migrate(&self) -> () {
        MIGRATOR
            .run(&self.pool)
            .await
            .expect("Faield to run migrator")
    }
}
