use std::sync::Arc;

use pictxn_backend::{
    database::{provider::Database, sqlite::db::SqliteDatabase},
    di::AppContext,
    result::Result,
    scout::{channels::safebooru::SafebooruChannel, service::Scout},
    server::{ServerConfig, configure_server},
    storage::{native::NativeFS, provider::FileStorage},
};

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
    let db = Database::new(sqlite);

    // 2. Initializing file storage
    let fs = NativeFS::new("./data/media", "./data/temp");
    fs.init()?;
    let storage = FileStorage::new(fs);

    // 3. Initializing scout
    let http_client = reqwest::Client::new();
    let safebooru_channel = SafebooruChannel::new(http_client.clone());
    let scout = Arc::new(Scout::new(100, 70, 20, 10).with_channel(safebooru_channel));
    scout.init().await;

    // 4. Collecting the application context and config
    let ctx = AppContext::new(db, storage, scout);
    let cfg = ServerConfig {
        host_addrs: "0.0.0.0:8080",
        use_open_api: true,
        ..ServerConfig::default()
    };

    // 5. Setting up the server
    let server = configure_server(ctx, cfg).await?;

    // 6. Start the server
    server.await?;
    Ok(())
}
