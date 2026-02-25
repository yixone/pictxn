use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use tokio::sync::RwLock;

use crate::scout::external_content::model::ExternalContent;

type PoolItem = ExternalContent;

#[derive(Debug)]
pub struct ScoutPool {
    /// Pool contents
    pub items: RwLock<VecDeque<Arc<PoolItem>>>,

    /// Pool size
    pub size: AtomicUsize,

    /// Max pool size
    pub max_size: usize,

    /// Minimum pool size after which filling begins
    pub low_size: usize,

    /// The critical pool size after which synchronous filling begins
    pub crit_size: usize,
}

impl ScoutPool {
    pub async fn push(&self, items: Vec<PoolItem>) {
        let items_iter = items.into_iter().map(Arc::new);

        let mut lock = self.items.write().await;
        lock.extend(items_iter);

        if lock.len() > self.max_size {
            let redutant = lock.len() - self.max_size;
            lock.drain(0..redutant);
        }
        self.size.store(lock.len(), Ordering::Relaxed);
    }

    pub async fn pull(&self, cursor: usize, limit: usize) -> Arc<[Arc<PoolItem>]> {
        let lock = self.items.read().await;
        if cursor >= lock.len() || limit == 0 {
            return Arc::new([]);
        }

        lock.iter().skip(cursor).take(limit).cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub async fn trim(&self, count: usize) {
        let mut lock = self.items.write().await;
        lock.drain(0..count);
        self.size.store(lock.len(), Ordering::Relaxed);
    }

    pub async fn snapshot(&self) -> Arc<[Arc<PoolItem>]> {
        let lock = self.items.read().await;
        lock.iter().cloned().collect()
    }
}
