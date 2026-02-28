use actix_web::{HttpResponse, get, web};

use crate::{di::AppContext, result::Result};

#[get("/discover")]
pub async fn discover_feed(ctx: web::Data<AppContext>) -> Result<HttpResponse> {
    let scout = &ctx.scout;

    let feed_slice = scout.next().await;
    Ok(HttpResponse::Ok().json(feed_slice))
}
