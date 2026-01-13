use std::{sync::Arc, time::Instant};

pub enum CacheExpiration {
    Never,
    At(Instant),
}

pub struct CacheItem<T> {
    value: Arc<T>,
    expired: CacheExpiration,
}

impl<T> CacheItem<T> {
    pub fn new(value: T, expired: CacheExpiration) -> Self {
        let value_arc = Arc::new(value);
        CacheItem {
            value: value_arc,
            expired,
        }
    }

    pub fn value(&self) -> Arc<T> {
        self.value.clone()
    }

    pub fn expired(&self) -> &CacheExpiration {
        &self.expired
    }
}
