use reqwest::Client;

pub struct ScoutService {
    client: Client,
}

impl ScoutService {
    pub fn new(client: Client) -> ScoutService {
        ScoutService { client }
    }

    /// Get a list of cards from external APIs
    pub async fn fetch(&self) {
        todo!()
    }
}
