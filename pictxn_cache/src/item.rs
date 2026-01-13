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
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn expired(&self) -> &CacheExpiration {
        &self.expired
    }
}
