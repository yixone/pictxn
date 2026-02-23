use actix_web::{App, HttpServer, dev::Server, web};

use crate::{di::AppContext, result::Result};

pub async fn configure_server(ctx: AppContext, cfg: ServerConfig) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new().configure(|cfg| {
            cfg.app_data(web::Data::new(ctx.to_owned()));
        })
    })
    .workers(cfg.workers_count)
    .bind(cfg.host_addrs)?
    .run();

    Ok(server)
}

/// Server config
pub struct ServerConfig {
    /// Server listen address
    pub host_addrs: &'static str,
    /// Enable OpenApi endpoints on startup
    pub use_open_api: bool,
    /// Number of server threads
    pub workers_count: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_addrs: "0.0.0.0:8080",
            use_open_api: false,
            workers_count: 6,
        }
    }
}
