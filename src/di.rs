use std::{ops::Deref, sync::Arc};

use crate::{database::provider::Database, storage::provider::FileStorage};

/// Application context for DI
#[derive(Clone)]
pub struct AppContext {
    inner: Arc<ContextInner>,
}

impl AppContext {
    pub fn new(database: Database, storage: FileStorage) -> Self {
        let inner = ContextInner { database, storage };
        AppContext {
            inner: Arc::new(inner),
        }
    }
}

impl Deref for AppContext {
    type Target = ContextInner;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

pub struct ContextInner {
    /// Database abstraction
    pub database: Database,

    /// File storage abstraction
    pub storage: FileStorage,
}
