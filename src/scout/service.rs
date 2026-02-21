use std::sync::Arc;

use crate::scout::channels::base::BaseChannel;

type AbstractChannel = Arc<dyn BaseChannel>;
pub struct ScoutService {
    channels: Vec<AbstractChannel>,
}

// TODO: add rate limiter
impl ScoutService {
    pub fn new(channels: Vec<AbstractChannel>) -> ScoutService {
        ScoutService { channels }
    }

    /// Get a list of cards from external APIs
    pub async fn fetch(&self) {
        todo!()
    }
}
