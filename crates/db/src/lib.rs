pub mod ops;

pub mod sqlite;

use std::{ops::Deref, sync::Arc};

use crate::ops::AbstractBase;

#[derive(Clone)]
pub struct Database {
    inner: Arc<dyn AbstractBase>,
}

impl Database {
    pub fn new<T: AbstractBase + 'static>(inner: T) -> Self {
        let inner = Arc::new(inner);
        Database { inner }
    }
}

impl Deref for Database {
    type Target = dyn AbstractBase;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}
