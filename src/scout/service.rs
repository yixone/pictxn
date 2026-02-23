use std::sync::Arc;

use rand::seq::SliceRandom;
use tracing::error;

use crate::{
    result::Result,
    scout::{channels::base::BaseChannel, content::ScoutContentItem},
};

pub(super) const CHANNEL_REQUEST_TIMEOUT: u64 = 4;

type AbstractChannel = Arc<dyn BaseChannel>;

#[derive(Clone)]
pub struct ScoutService {
    channels: Vec<AbstractChannel>,
}

// TODO: add rate limiter
impl ScoutService {
    pub fn new(channels: Vec<AbstractChannel>) -> ScoutService {
        ScoutService { channels }
    }

    /// Get a list of cards from external API
    pub async fn fetch(&self, limit: u32, pid: u32) -> Result<Vec<ScoutContentItem>> {
        if limit == 0 || self.channels.is_empty() {
            return Ok(vec![]);
        }

        let channels_count = self.channels.len() as u32;
        let limit_per_channel = limit.div_ceil(channels_count).max(1) * 2;

        let futures = self
            .channels
            .iter()
            .map(|c| fetch_task(c.clone(), limit_per_channel, pid));
        let result = futures::future::join_all(futures).await;

        let mut items = result.into_iter().flatten().collect::<Vec<_>>();

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
