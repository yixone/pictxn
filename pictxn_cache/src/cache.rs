use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use crate::item::{CacheExpiration, CacheItem};

pub(super) struct CacheInner<K, V> {
    map: RwLock<HashMap<K, CacheItem<V>>>,
}

impl<K, V> CacheInner<K, V>
where
    K: Eq + Hash,
{
    pub(super) fn new() -> Self {
        let map = HashMap::new();
        let locked_map = RwLock::new(map);

        CacheInner { map: locked_map }
    }

    pub(super) fn get(&self, key: K) -> Option<Arc<V>> {
        let lock = self.map.read().unwrap();

        let value = lock.get(&key)?;
        match value.expired() {
            CacheExpiration::Never => Some(value.value()),
            CacheExpiration::At(i) => {
                if &Instant::now() > i {
                    drop(lock);
                    self.remove(key);
                    None
                } else {
                    Some(value.value())
                }
            }
        }
    }

    pub(super) fn set(&self, key: K, value: V) {
        let value = CacheItem::new(value, CacheExpiration::Never);

        {
            let mut write_lock = self.map.write().unwrap();

            write_lock.insert(key, value);
        }
    }

    pub(super) fn set_ex(&self, key: K, value: V, lifetime: Duration) {
        let expired = CacheExpiration::At(Instant::now() + lifetime);
        let value = CacheItem::new(value, expired);

        {
            let mut write_lock = self.map.write().unwrap();

            write_lock.insert(key, value);
        }
    }

    pub(super) fn remove(&self, key: K) {
        {
            let mut write_lock = self.map.write().unwrap();

            write_lock.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn set_and_get_value_by_key() {
        let cache = CacheInner::new();

        let key = "key".to_string();
        let value = "value".to_string();

        cache.set(key.clone(), value.clone());

        let expected = Arc::new(value);

        let v1 = cache.get(key.clone()).unwrap();
        let v2 = cache.get(key).unwrap();

        assert_eq!(
            v1, expected,
            "The received value must match the inserted one"
        );

        assert_eq!(
            v2, expected,
            "The received value must match the inserted one"
        );

        assert!(
            Arc::ptr_eq(&v1, &v2),
            "Values for one key must lead to one Arc"
        );
    }

    #[test]
    fn overvrite_value_by_key() {
        let cache = CacheInner::new();

        let key = "key".to_string();

        let value = "value".to_string();

        cache.set(key.clone(), value.clone());
        let v1 = cache.get(key.clone()).unwrap();
        assert_eq!(v1, Arc::new(value));

        let value2 = "another_value".to_string();

        cache.set(key.clone(), value2.clone());
        let v2 = cache.get(key).unwrap();
        assert_eq!(v2, Arc::new(value2));
    }

    #[test]
    fn set_ex_and_get_value_by_key() {
        let cache = CacheInner::new();

        let key = "key".to_string();
        let value = "value".to_string();

        cache.set_ex(key.clone(), value.clone(), Duration::from_secs(5));

        let v1 = cache.get(key.clone()).unwrap();

        assert_eq!(v1, Arc::new(value));
    }

    #[test]
    fn return_none_for_not_existed_value() {
        let cache = CacheInner::new();

        let key = "key".to_string();
        let value = "value".to_string();

        cache.set(key.clone(), value.clone());

        let new_key = "new-key".to_string();

        let v1 = cache.get(new_key);

        assert!(v1.is_none());
    }

    #[test]
    fn return_none_for_expired_value() {
        let cache = CacheInner::new();

        let key = "key".to_string();
        let value = "value".to_string();

        cache.set_ex(key.clone(), value.clone(), Duration::from_secs(1));

        let v1 = cache.get(key.clone());
        assert!(v1.is_some());

        sleep(Duration::from_secs(1));

        let v2 = cache.get(key.clone());
        assert!(v2.is_none());
    }

    #[test]
    fn remove_value_by_key() {
        let cache = CacheInner::new();

        let key = "key".to_string();
        let value = "value".to_string();

        cache.set(key.clone(), value.clone());

        let v1 = cache.get(key.clone());
        assert!(v1.is_some());

        cache.remove(key.clone());

        let v2 = cache.get(key.clone());
        assert!(v2.is_none());
    }
}
