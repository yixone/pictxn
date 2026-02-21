use std::{ops::Deref, sync::Arc};

use crate::database::ops::AbstractDatabase;

/// Abstract database provider
#[derive(Clone)]
pub struct Database {
    inner: Arc<dyn AbstractDatabase>,
}

impl Database {
    pub fn new<T>(db: T) -> Self
    where
        T: AbstractDatabase + 'static,
    {
        let inner = Arc::new(db);
        Self { inner }
    }
}

impl Deref for Database {
    type Target = dyn AbstractDatabase;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}
