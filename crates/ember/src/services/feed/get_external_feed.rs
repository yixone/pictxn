use result::Result;
use scout::{ScoutService, models::cards::ScoutCard};
use tracing::info;

pub struct DataGetExternalFeed {
    /// Number of cards in the response
    batch_size: u32,

    /// Page id
    pid: u32,
}

/// Collect a feed of cards from external APIs
pub async fn get_external_feed(
    data: &DataGetExternalFeed,
    scout: &ScoutService,
) -> Result<Vec<ScoutCard>> {
    let cards = scout.fetch(data.batch_size, data.pid).await?;

    info!(
        cards = cards.len(),
        batch = data.batch_size,
        pid = data.pid,
        "Fetched external feed"
    );

    Ok(cards)
}
