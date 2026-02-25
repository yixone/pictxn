use std::{ops::Deref, sync::Arc};

use crate::{database::provider::Database, storage::provider::FileStorage};

/// Application context for DI
#[derive(Clone)]
pub struct AppContext {
    inner: Arc<ContextInner>,
}

impl AppContext {
    pub fn new(db: Database, storage: FileStorage) -> Self {
        let inner = ContextInner { db, storage };
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
    /// Main database
    pub db: Database,

    /// File storage abstraction
    pub storage: FileStorage,
}
