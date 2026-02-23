/// Get discover_feed: `/feed/discover`
mod discover_feed;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/feed").service(discover_feed::get_discover_feed));
}
