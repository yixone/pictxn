use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

pub(super) struct CacheInner<K, V> {
    map: RwLock<HashMap<K, Arc<V>>>,
}

impl<K, V> CacheInner<K, V> {
    pub fn get(&self, key: K) -> Option<V> {
        todo!()
    }

    pub fn set(&self, key: K, value: V) -> Option<V> {
        todo!()
    }

    pub fn set_ex(&self, key: K, value: V, lifetime: Duration) -> Option<V> {
        todo!()
    }

    pub fn remove(&self, key: K) -> Option<V> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    fn set_and_get_value_by_key() {
        //
    }

    #[cfg(test)]
    fn overvrite_value_by_key() {
        //
    }

    #[cfg(test)]
    fn set_ex_and_get_value_by_key() {
        //
    }

    #[cfg(test)]
    fn return_none_for_not_existed_value() {
        //
    }

    #[cfg(test)]
    fn return_none_for_not_expired_value() {
        //
    }

    #[cfg(test)]
    fn remove_value_by_key() {
        //
    }
}
