/// DTO and API projections
pub mod api;
/// Database module
pub mod database;
/// Domain models and types
pub mod domains;
/// Result and Error types
pub mod result;
/// External APIs content aggregator
pub mod scout;
/// Business logic and use cases
pub mod services;
/// File storage services
pub mod storage;
/// Utilities
pub mod util;

/// Application context for DI
#[derive(Clone)]
pub struct AppContext {
    /// Database abstraction
    pub database: database::provider::Database,

    /// File storage abstraction
    pub storage: storage::provider::FileStorage,

    /// Scout feed service
    pub scout: scout::service::ScoutService,
}
