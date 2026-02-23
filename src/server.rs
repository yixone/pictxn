use actix_web::{App, HttpServer, dev::Server, web};

use crate::{di::AppContext, result::Result};

pub struct ServerConfig {
    /// Server listen address
    pub host_addrs: &'static str,
    /// Enable OpenApi endpoints on startup
    pub use_open_api: bool,
}

pub async fn configure_server(ctx: AppContext, cfg: ServerConfig) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new().configure(|cfg| {
            cfg.app_data(web::Data::new(ctx.to_owned()));
        })
    })
    .bind(cfg.host_addrs)?
    .run();

    Ok(server)
}
