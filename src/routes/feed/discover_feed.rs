use actix_web::{HttpResponse, get, web};

use crate::{di::AppContext, result::Result, services};

#[get("/discover")]
pub async fn get_discover_feed(ctx: web::Data<AppContext>) -> Result<HttpResponse> {
    let items = services::feed::discover_feed(0, 15, &ctx.scout).await?;
    Ok(HttpResponse::Ok().json(items))
}
