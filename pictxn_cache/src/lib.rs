use std::sync::Arc;

use crate::cache::CacheInner;

mod cache;

mod item;

pub struct Cache<K, V> {
    inner: Arc<CacheInner<K, V>>,
}
