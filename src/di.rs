use crate::{
    database::provider::Database, scout::service::ScoutService, storage::provider::FileStorage,
};

/// Application context for DI
#[derive(Clone)]
pub struct AppContext {
    /// Database abstraction
    pub database: Database,

    /// File storage abstraction
    pub storage: FileStorage,

    /// Scout feed service
    pub scout: ScoutService,
}
