use std::{ops::Deref, sync::Arc};

use crate::{
    database::provider::Database, scout::service::ScoutService, storage::provider::FileStorage,
};

/// Application context for DI
#[derive(Clone)]
pub struct AppContext {
    inner: Arc<ContextInner>,
}

impl AppContext {
    pub fn new(database: Database, storage: FileStorage, scout: ScoutService) -> Self {
        let inner = ContextInner {
            database,
            storage,
            scout,
        };
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

    /// Scout feed service
    pub scout: ScoutService,
}
