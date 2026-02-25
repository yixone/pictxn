use std::fmt::Debug;

use crate::{result::Result, scout::external_content::model::ExternalContent};

#[async_trait::async_trait]
pub trait BaseChannel: Send + Sync + Debug {
    /// Fetch list of items from channel
    async fn fetch(&self, limit: u32) -> Result<Vec<ExternalContent>>;
}
