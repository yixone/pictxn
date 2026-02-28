/// DTO and API projections
pub mod api;
/// Database module
pub mod database;
/// Domain models and types
pub mod domains;
/// Result and Error types
pub mod result;
/// Backend HTTP endpoints
pub mod routes;
/// External APIs content aggregator
pub mod scout;
/// File storage services
pub mod storage;
/// Background tasks
pub mod tasks;
/// Utilities
pub mod util;

/// Application context for DI
pub mod di;
/// Backend server assembly and configuration
pub mod server;
