use std::{ops::Deref, sync::Arc};

use crate::{database::provider::Database, scout::service::Scout, storage::provider::FileStorage};

/// Application context for DI
#[derive(Clone)]
pub struct AppContext {
    inner: Arc<ContextInner>,
}

impl AppContext {
    pub fn new(db: Database, storage: FileStorage, scout: Arc<Scout>) -> Self {
        let inner = ContextInner { db, storage, scout };
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

    /// Scout service
    pub scout: Arc<Scout>,
}
