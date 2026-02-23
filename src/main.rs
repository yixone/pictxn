use std::sync::Arc;

use pictxn_backend::{
    database::{provider::Database, sqlite::db::SqliteDatabase},
    di::AppContext,
    result::Result,
    scout::{channels::safebooru::SafebooruChannel, service::ScoutService},
    server::{ServerConfig, configure_server},
    storage::{native::NativeFS, provider::FileStorage},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // 0. Initializing logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .init();

    // 1. Initializing the database
    let sqlite = SqliteDatabase::open_file("./data/app.db").await?;
    sqlite.migrate().await?;
    let database = Database::new(sqlite);

    // 2. Initializing file storage
    let fs = NativeFS::new("./data/media", "./data/temp");
    fs.init()?;
    let storage = FileStorage::new(fs);

    // 3. Initializing channels for Scout
    let http_client = Client::new();
    let safebooru = Arc::new(SafebooruChannel::new(http_client.clone()));
    let scout = ScoutService::new(vec![safebooru]);

    // 4. Collecting the application context and config
    let ctx = AppContext::new(database, storage, scout);
    let cfg = ServerConfig {
        host_addrs: "0.0.0.0:8080",
        use_open_api: true,
        ..Default::default()
    };

    // 5. Setting up the server
    let server = configure_server(ctx, cfg).await?;

    // 6. Start the server
    server.await?;
    Ok(())
}
