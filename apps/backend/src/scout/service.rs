use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use rand::seq::SliceRandom;
use tokio::sync::{Mutex, Notify, RwLock};
use tracing::{error, info};

use crate::scout::{channels::ops::AbstractChannel, model::ScoutCard, pool::ScoutPool};

#[derive(Debug)]
pub struct Scout {
    channels: Vec<Arc<dyn AbstractChannel>>,
    head: AtomicUsize,
    global_head: AtomicUsize,
    pool: Arc<ScoutPool>,
    next_batch: RwLock<Arc<[Arc<ScoutCard>]>>,
    batch_size: usize,
    watermark: usize,
    target: usize,
    next_guard: Mutex<()>,
    state: Mutex<ScoutState>,
}

#[derive(Debug)]
enum ScoutState {
    Idle,
    Refilling { notify: Arc<Notify> },
}

impl Scout {
    pub fn new(target: usize, watermark: usize, batch_size: usize) -> Self {
        Scout {
            channels: Vec::new(),
            head: AtomicUsize::new(0),
            global_head: AtomicUsize::new(0),
            pool: Arc::new(ScoutPool::new(target)),
            next_batch: RwLock::new(Arc::new([])),
            batch_size,
            target,
            watermark,
            next_guard: Mutex::new(()),
            state: Mutex::new(ScoutState::Idle),
        }
    }

    pub fn with_channel<T>(mut self, channel: T) -> Self
    where
        T: AbstractChannel + 'static,
    {
        self.channels.push(Arc::new(channel));
        self
    }

    pub async fn init(&self) {
        if self.batch_size > self.target {
            panic!("batch_size cannot exceed the target");
        }

        if self.channels.is_empty() {
            panic!("scout cannot be used without specifying any channels");
        }

        if self.watermark > self.target || self.watermark <= self.batch_size {
            panic!("watermark must be in the range from batch_size to target");
        }

        info!(
            channels_count = self.channels.len(),
            pool_size = self.target,
            batch_size = self.batch_size,
            "scout initialized"
        );
    }

    async fn refill(&self, count: usize) {
        let mut state = self.state.lock().await;
        let notify = Arc::new(Notify::new());
        *state = ScoutState::Refilling {
            notify: notify.clone(),
        };
        drop(state);

        let limit_per_channel = (count / self.channels.len()) as u32;
        let mut items = futures::future::join_all(self.channels.iter().map(async |cnl| {
            match cnl.fetch(limit_per_channel).await {
                Ok(items) => items,
                Err(err) => {
                    error!(err =?err, "scout.channel error");
                    Vec::new()
                }
            }
        }))
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        items.shuffle(&mut rand::rng());
        items.truncate(count);

        {
            let mut state = self.state.lock().await;
            self.pool.push(items).await;
            *state = ScoutState::Idle;
        }
        notify.notify_waiters();
    }

    fn global_head(&self) -> usize {
        self.global_head.load(Ordering::Acquire)
    }

    fn move_global_head(&self, move_by: usize) {
        self.global_head.fetch_add(move_by, Ordering::Release);
    }

    fn head(&self) -> usize {
        self.head.load(Ordering::Acquire)
    }

    fn move_head(&self, move_by: usize) {
        self.head.fetch_add(move_by, Ordering::Release);
    }

    async fn update_next_batch(&self) {
        let head = self.head.load(Ordering::Acquire);
        let g_head = self.global_head.load(Ordering::Acquire);

        let l_head = head - g_head;
        let mut next_batch = self.pool.pull(l_head, self.batch_size).await;
        next_batch.shuffle(&mut rand::rng());

        *self.next_batch.write().await = Arc::from_iter(next_batch);
    }

    async fn exclude_empty_pool(&self) {
        let pool_len = self.pool.len().await;
        if pool_len <= self.batch_size {
            self.refill(self.target - pool_len).await;
            self.update_next_batch().await;
            info!(
                refilling = self.target - pool_len,
                "scout pool is less than batch_size"
            );
        }
    }

    async fn check_refill(self: &Arc<Self>) {
        let l_head = self.head() - self.global_head();
        let remaining = self.target - (l_head + self.batch_size);

        let state = self.state.lock().await;

        match &*state {
            ScoutState::Refilling { notify } => {
                let notify = notify.clone();
                drop(state);

                info!(reason = "already refilling", "scout refill cancelling");
                notify.notified().await;
                return;
            }
            ScoutState::Idle => drop(state),
        }

        if remaining <= self.batch_size {
            let removed = self.pool.trim(l_head + self.batch_size).await;

            info!(
                refill_count = removed,
                reason = "critical watermark reached",
                "scout refill calling"
            );
            self.move_global_head(removed);
            self.refill(removed).await;
        } else if remaining <= self.watermark {
            let this = self.clone();
            tokio::spawn(async move {
                let removed = this.pool.trim(l_head + this.batch_size).await;

                info!(
                    refill_count = removed,
                    reason = "low watermark reached",
                    "scout refill calling"
                );
                this.move_global_head(removed);
                this.refill(removed).await;
            });
        }
    }

    pub async fn next(self: &Arc<Self>) -> Arc<[Arc<ScoutCard>]> {
        let _guard = self.next_guard.lock().await;
        info!("scout.next - called");

        self.exclude_empty_pool().await;

        let current_batch = self.next_batch.read().await.clone();

        self.check_refill().await;

        // self.move_head(self.batch_size);
        self.update_next_batch().await;

        current_batch
    }
}
