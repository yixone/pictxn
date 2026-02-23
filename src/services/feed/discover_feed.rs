use crate::{
    result::{Result, errors::AppError},
    scout::{self, service::ScoutService},
};

/// The maximum number of cards that can be fetched in one request
const MAX_LIMIT: u32 = 512;

/// Build a Discover feed from external APIs
pub async fn discover_feed(
    pid: u32,
    limit: u32,
    scout: &ScoutService,
) -> Result<Vec<scout::content::ScoutContentItem>> {
    let items = scout.fetch(limit.min(MAX_LIMIT), pid).await?;
    if items.is_empty() {
        return Err(AppError::EmptyFeed);
    }
    Ok(items)
}
