use std::{sync::Arc, time::Instant};

use chrono::{Duration, Utc};
use rand::seq::SliceRandom;
use tracing::{debug, error, info};

use crate::{
    database::provider::Database,
    result::Result,
    scout::{channels::base::BaseChannel, external_content::model::ExternalContent},
    tasks::task::BackgroundTask,
};

/// Target number of cards in the pool
const TARGET_CONTENT: u32 = 500;
/// Minimum number of cards allowed in a pool
const MIN_CONTENT: u32 = 350;
/// Maximum number of cards per request
const MAX_BATCH_SIZE: u32 = MIN_CONTENT / 10;
/// The time (in sec) after which old cards will be deleted
const EXTERNAL_CONTENT_TTL: i64 = 45 * 60;
/// Random time deviation to remove full pool flushes and randomize refreshes
pub(super) const RANDOM_TTL_OFFSET: i64 = EXTERNAL_CONTENT_TTL / 4;
/// Time (in sec) between service cycles
const CYCLE_COOLDOWN: u64 = 25;

#[derive(Clone)]
pub struct ScoutTask {
    channels: Vec<Arc<dyn BaseChannel>>,
    db: Database,
}

impl ScoutTask {
    pub fn new(db: Database) -> Self {
        ScoutTask {
            channels: vec![],
            db,
        }
    }

    pub fn with_channel<T>(mut self, channel: T) -> Self
    where
        T: BaseChannel + Send + Sync + 'static,
    {
        self.channels.push(Arc::new(channel));
        self
    }

    async fn fetch_from_channels(&self, limit: u32) -> Result<Vec<ExternalContent>> {
        if limit == 0 || self.channels.is_empty() {
            return Ok(vec![]);
        }

        let channels_count = self.channels.len() as u32;
        let limit_per_channel = limit.div_ceil(channels_count).max(1);

        let futures = self
            .channels
            .iter()
            .map(async |c| match c.fetch(limit_per_channel).await {
                Ok(i) => i,
                Err(e) => {
                    error!(err = ?e, "Scout Provider error");
                    Vec::new()
                }
            });
        let res = futures::future::join_all(futures).await;
        let mut items = res.into_iter().flatten().collect::<Vec<_>>();

        items.shuffle(&mut rand::rng());
        items.truncate(limit as usize);

        Ok(items)
    }

    async fn try_remove_old_content(&self) -> Result<()> {
        let date = Utc::now() - Duration::seconds(EXTERNAL_CONTENT_TTL);
        debug!(date = ?date, "Removing ExternalContent created before");

        self.db
            .remove_old_external_content(date, MAX_BATCH_SIZE)
            .await
    }

    pub async fn execute(&self) -> Result<()> {
        info!("Scout cycle started!");
        let cycle_start_time = Instant::now();

        self.try_remove_old_content().await?;

        let content_count = self.db.count_external_content().await?;
        info!(count = content_count, "ExternalContentPool contains");

        if content_count > MIN_CONTENT {
            info!("The ScoutTask cycle was skipped because the number goal was reached");
            return Ok(());
        }

        let d_count = (TARGET_CONTENT - content_count).min(MAX_BATCH_SIZE);
        info!(
            count = TARGET_CONTENT - d_count,
            "The External Content pool is missing items"
        );

        let items = self.fetch_from_channels(d_count).await?;
        info!(
            fetched = items.len(),
            requested = d_count,
            "Scout fetch completed"
        );
        self.db.insert_external_content_many(&items).await?;

        info!(
            cycle_duration_ms = cycle_start_time.elapsed().as_millis(),
            "Scout cycle finished!"
        );
        Ok(())
    }
}

#[async_trait::async_trait]
impl BackgroundTask for ScoutTask {
    async fn run(&self) {
        loop {
            if let Err(e) = self.execute().await {
                error!(err = ?e, "An error occurred with ScoutTask");
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(CYCLE_COOLDOWN)).await;
        }
    }
}
