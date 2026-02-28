use std::{ops::Deref, sync::Arc};

use crate::storage::ops::AbstractFileStorage;

/// Abstract file storage provider
#[derive(Clone)]
pub struct FileStorage {
    inner: Arc<dyn AbstractFileStorage>,
}

impl FileStorage {
    pub fn new<T>(fs: T) -> Self
    where
        T: AbstractFileStorage + 'static,
    {
        let inner = Arc::new(fs);
        Self { inner }
    }
}

impl Deref for FileStorage {
    type Target = dyn AbstractFileStorage;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}
