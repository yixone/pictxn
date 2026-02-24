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

const TARGET_CONTENT: u32 = 50;
const MIN_CONTENT: u32 = 15;
const MAX_BATCH_SIZE: u32 = 100;
const EXTERNAL_CONTENT_TTL: i64 = 5 * 60 * 60;

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

        // TODO: remove PID param from scout service
        let futures =
            self.channels
                .iter()
                .map(async |c| match c.fetch(limit_per_channel, 1).await {
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
        debug!("Scout cycle started!");
        let cycle_start_time = Instant::now();

        self.try_remove_old_content().await?;

        let content_count = self.db.count_external_content().await?;
        debug!(count = content_count, "ExternalContentPool contains");

        if content_count > MIN_CONTENT {
            tracing::debug!("The ScoutTask cycle was skipped because the number goal was reached");
            return Ok(());
        }

        let d_count = (TARGET_CONTENT - content_count).min(MAX_BATCH_SIZE);
        info!(
            count = d_count,
            "The External Content pool is missing items"
        );

        let items = self.fetch_from_channels(d_count).await?;
        info!(
            fetched = items.len(),
            requested = d_count,
            "Scout fetch completed"
        );
        self.db.insert_external_content_many(&items).await?;

        debug!(
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

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }
}
