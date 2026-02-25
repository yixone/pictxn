use std::{
    collections::VecDeque,
    sync::{Arc, atomic::AtomicUsize},
};

use rand::seq::SliceRandom;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info, warn};

use crate::scout::{
    channels::base::BaseChannel, external_content::model::ExternalContent, pool::ScoutPool,
};

#[derive(Debug)]
pub struct Scout {
    channels: Vec<Arc<dyn BaseChannel>>,

    pool: ScoutPool,

    batch_size: usize,

    refill_lock: Arc<Mutex<()>>,
}

impl Scout {
    pub fn new(max_size: usize, low_size: usize, crit_size: usize, batch_size: usize) -> Self {
        Scout {
            channels: vec![],
            pool: ScoutPool {
                items: RwLock::new(VecDeque::new()),
                size: AtomicUsize::new(0),
                max_size,
                low_size,
                crit_size,
            },
            batch_size,
            refill_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn with_channel<T>(mut self, c: T) -> Self
    where
        T: BaseChannel + 'static,
    {
        self.channels.push(Arc::new(c));
        self
    }

    pub async fn init(self: &Arc<Self>) {
        let this = self.clone();
        tokio::spawn(async move {
            this.refill(this.pool.max_size / 2).await;
        });
    }

    async fn refill(&self, count: usize) {
        let _guard = self.refill_lock.lock().await;
        if self.pool.len() > self.pool.low_size || self.channels.is_empty() {
            warn!(
                reason = "enought_pool_data",
                pool_size = self.pool.len(),
                "scout.refill cancelled"
            );
            return;
        }

        let limit_per_channel = count as u32;
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
        items.truncate(count);

        self.pool.push(items).await;
        info!(
            fetched = count,
            pool_len = self.pool.len(),
            "scout.refill completed"
        );
    }

    pub async fn pull(self: &Arc<Self>, cursor: usize) -> Arc<[Arc<ExternalContent>]> {
        let pool_len = self.pool.len();

        let cursor = cursor % pool_len;
        let remaining = pool_len - (cursor + self.batch_size);

        info!(
            cursor = cursor,
            batch = self.batch_size,
            remaining = remaining,
            pool_len = pool_len,
            "scout.pull called"
        );

        if remaining <= self.pool.crit_size {
            warn!(
                reason = "critical_watermark",
                pool_len = pool_len,
                "scout.refill started"
            );
            self.refill(self.pool.max_size - self.pool.len()).await;
        } else if remaining <= self.pool.low_size {
            let this = self.clone();
            tokio::spawn(async move {
                let refill_count = this.batch_size * 2;
                info!(
                    reason = "low_watermark",
                    pool_len = pool_len,
                    refill_count = refill_count,
                    "scout.refill started"
                );
                this.refill(refill_count).await;
            });
        }

        self.pool.pull(cursor, self.batch_size).await
    }
}
