use std::{collections::VecDeque, sync::Arc};

use tokio::sync::RwLock;

use crate::scout::model::ScoutCard;

#[derive(Debug)]
pub struct ScoutPool {
    inner: RwLock<VecDeque<Arc<ScoutCard>>>,
}

impl ScoutPool {
    pub fn new(max_size: usize) -> Self {
        ScoutPool {
            inner: RwLock::new(VecDeque::with_capacity(max_size * 2)),
        }
    }

    pub async fn push(&self, items: Vec<ScoutCard>) {
        let items = items.into_iter().map(Arc::new);

        let mut lock = self.inner.write().await;
        lock.extend(items);
    }

    pub async fn pull(&self, cursor: usize, limit: usize) -> Vec<Arc<ScoutCard>> {
        let lock = self.inner.read().await;
        if cursor >= lock.len() || limit == 0 {
            return Vec::new();
        }

        lock.iter().skip(cursor).take(limit).cloned().collect()
    }

    pub async fn len(&self) -> usize {
        let lock = self.inner.read().await;
        lock.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    pub async fn trim(&self, count: usize) -> usize {
        let mut lock = self.inner.write().await;
        let removed = lock.drain(0..count);
        removed.len()
    }
}
