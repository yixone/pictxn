use std::path::PathBuf;

use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::{
    database::ops::AbstractDatabase,
    result::{Result, errors::AppError},
};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct SqliteDatabase {
    pub(super) pool: SqlitePool,
}

impl SqliteDatabase {
    /// Open database from file
    pub async fn open_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
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
    pub async fn migrate(&self) -> Result<()> {
        MIGRATOR
            .run(&self.pool)
            .await
            .map_err(|e| AppError::from(sqlx::Error::from(e)))
    }
}

impl AbstractDatabase for SqliteDatabase {}
