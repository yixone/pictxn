use actix_web::{HttpResponse, get, web};
use serde::Deserialize;

use crate::{di::AppContext, result::Result, services};

const DEFAULT_LIMIT: u32 = 50;

#[derive(Deserialize)]
pub struct QueryGetDiscoverFeed {
    pid: Option<u32>,
    limit: Option<u32>,
}

#[get("/discover")]
pub async fn get_discover_feed(
    ctx: web::Data<AppContext>,
    params: web::Query<QueryGetDiscoverFeed>,
) -> Result<HttpResponse> {
    let items = services::feed::discover_feed(
        params.pid.unwrap_or(0),
        params.limit.unwrap_or(DEFAULT_LIMIT),
        &ctx.scout,
    )
    .await?;
    Ok(HttpResponse::Ok().json(items))
}
