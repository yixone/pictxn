use actix_web::{HttpResponse, get, web};
use serde::Deserialize;

use crate::{di::AppContext, result::Result};

const DEFAULT_LIMIT: u32 = 50;

#[derive(Deserialize)]
pub struct QueryGetDiscoverFeed {
    pid: Option<u32>,
    limit: Option<u32>,
}

#[get("/discover")]
pub async fn discover_feed(
    ctx: web::Data<AppContext>,
    params: web::Query<QueryGetDiscoverFeed>,
) -> Result<HttpResponse> {
    todo!();
}
