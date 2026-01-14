use std::{sync::Arc, time::Duration};

use crate::cache::CacheInner;

mod cache;

mod item;

pub struct Cache<K, V> {
    inner: Arc<CacheInner<K, V>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: K) -> Option<Arc<V>> {
        self.inner.get(key)
    }

    pub fn set(&self, key: K, value: V) {
        self.inner.set(key, value)
    }

    pub fn set_ex(&self, key: K, value: V, lifetime: Duration) {
        self.inner.set_ex(key, value, lifetime)
    }

    pub fn remove(&self, key: K) {
        self.inner.remove(key)
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn default() -> Self {
        let inner = CacheInner::new();
        Cache {
            inner: Arc::new(inner),
        }
    }
}
