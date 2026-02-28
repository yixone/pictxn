use std::fmt::Debug;

use crate::{result::Result, scout::model::ScoutCard};

#[async_trait::async_trait]
pub trait AbstractChannel: Send + Sync + Debug {
    async fn fetch(&self, limit: u32) -> Result<Vec<ScoutCard>>;
}
