use std::sync::Arc;

use rand::seq::SliceRandom;
use tracing::error;

use crate::models::cards::ScoutCard;

pub mod providers;

pub mod errors;
pub mod models;

#[async_trait::async_trait]
pub trait ScoutProvider: Send + Sync {
    /// Get a list of files from a self source
    async fn fetch_content(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<Vec<models::cards::ScoutCard>, errors::ScoutError>;
}

pub struct ScoutService {
    providers: Vec<Arc<dyn ScoutProvider>>,
}

impl ScoutService {
    pub fn new(providers: Vec<Arc<dyn ScoutProvider>>) -> Self {
        Self { providers }
    }

    pub async fn fetch(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<Vec<models::cards::ScoutCard>, errors::ScoutError> {
        let fetch_tasks = self.providers.iter().map(|p| {
            let p = p.clone();
            tokio::spawn(service_task(p, limit, page))
        });

        let results = futures::future::join_all(fetch_tasks).await;
        let mut items = results
            .into_iter()
            .flat_map(|v| v.unwrap_or_default())
            .collect::<Vec<_>>();

        items.shuffle(&mut rand::rng());
        items.truncate(limit);

        Ok(items)
    }
}

async fn service_task(p: Arc<dyn ScoutProvider>, limit: usize, page: usize) -> Vec<ScoutCard> {
    let res = p.fetch_content(limit, page).await;
    match res {
        Ok(items) => items,
        Err(e) => {
            error!(err = ?e, "Scout Provider error");
            Vec::new()
        }
    }
}
