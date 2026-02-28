use actix_web::{App, HttpServer, dev::Server, web};

use crate::{di::AppContext, result::Result, routes};

pub async fn configure_server(ctx: AppContext, cfg: ServerConfig) -> Result<Server> {
    let use_open_api = cfg.use_open_api;

    let server = HttpServer::new(move || {
        App::new()
            .configure(|cfg| {
                // Base routes
                cfg.configure(routes::feed::configure);

                // System routes
                // TODO!

                // OpenApi routes
                if use_open_api {
                    // TODO!
                }
            })
            .app_data(web::Data::new(ctx.clone()))
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
