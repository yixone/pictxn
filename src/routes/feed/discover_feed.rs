use actix_web::{HttpResponse, get, web};
use serde::Deserialize;

use crate::{di::AppContext, result::Result};

#[derive(Deserialize)]
pub struct QueryGetDiscoverFeed {
    // cursor: Option<u32>,
}

#[get("/discover")]
pub async fn discover_feed(
    ctx: web::Data<AppContext>,
    params: web::Query<QueryGetDiscoverFeed>,
) -> Result<HttpResponse> {
    let scout = &ctx.scout;

    let feed_slice = scout.next().await;
    Ok(HttpResponse::Ok().json(feed_slice))
}
