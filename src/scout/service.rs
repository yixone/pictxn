use std::sync::Arc;

use rand::seq::SliceRandom;
use tracing::error;

use crate::{
    result::Result,
    scout::{channels::base::BaseChannel, content::ScoutContentItem},
};

type AbstractChannel = Arc<dyn BaseChannel>;
pub struct ScoutService {
    channels: Vec<AbstractChannel>,
}

// TODO: add rate limiter
impl ScoutService {
    pub fn new(channels: Vec<AbstractChannel>) -> ScoutService {
        ScoutService { channels }
    }

    async fn fetch_list(&self, limit: u32, pid: u32) -> Result<Vec<ScoutContentItem>> {
        let tasks = self
            .channels
            .iter()
            .map(|c| tokio::spawn(fetch_task(c.clone(), limit, pid)));

        Ok(futures::future::join_all(tasks)
            .await
            .into_iter()
            .flat_map(|i| i.unwrap_or_default())
            .collect())
    }

    /// Get a list of cards from external API
    pub async fn fetch(&self, limit: u32, pid: u32) -> Result<Vec<ScoutContentItem>> {
        let mut items = self.fetch_list(limit, pid).await?;

        items.shuffle(&mut rand::rng());
        items.truncate(limit as usize);

        Ok(items)
    }
}

async fn fetch_task(channel: AbstractChannel, limit: u32, pid: u32) -> Vec<ScoutContentItem> {
    match channel.fetch(limit, pid).await {
        Ok(items) => items,
        Err(e) => {
            error!(err = ?e, "Scout Provider error");
            Vec::new()
        }
    }
}
