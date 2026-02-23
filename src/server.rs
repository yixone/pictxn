use crate::di::AppContext;

pub struct ServerConfig {
    pub use_open_api: bool,
}

pub async fn configure_server(ctx: AppContext, cfg: ServerConfig) {
    todo!()
}
