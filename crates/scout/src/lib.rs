use std::sync::Arc;

use rand::seq::SliceRandom;
use result::Result;
use tracing::error;

use crate::models::cards::ScoutCard;

pub mod models;
pub mod providers;

#[async_trait::async_trait]
pub trait ScoutProvider: Send + Sync {
    /// Get a list of files from a self source
    async fn fetch_content(&self, limit: u32, page: u32) -> Result<Vec<models::cards::ScoutCard>>;
}

pub struct ScoutService {
    providers: Vec<Arc<dyn ScoutProvider>>,
}

impl ScoutService {
    pub fn new(providers: Vec<Arc<dyn ScoutProvider>>) -> Self {
        Self { providers }
    }

    pub async fn fetch(&self, limit: u32, page: u32) -> Result<Vec<models::cards::ScoutCard>> {
        let fetch_tasks = self
            .providers
            .iter()
            .map(|p| tokio::spawn(service_task(p.clone(), limit, page)));

        let results = futures::future::join_all(fetch_tasks).await;
        let mut items = results
            .into_iter()
            .flat_map(|v| v.unwrap_or_default())
            .collect::<Vec<_>>();

        items.shuffle(&mut rand::rng());
        items.truncate(limit as usize);

        Ok(items)
    }
}

async fn service_task(p: Arc<dyn ScoutProvider>, limit: u32, page: u32) -> Vec<ScoutCard> {
    let res = p.fetch_content(limit, page).await;
    match res {
        Ok(items) => items,
        Err(e) => {
            error!(err = ?e, "Scout Provider error");
            Vec::new()
        }
    }
}
