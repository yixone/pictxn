use serde::Serialize;

pub mod safebooru;

#[derive(Debug, Clone, Serialize)]
pub enum ProviderType {
    Safebooru,
}
