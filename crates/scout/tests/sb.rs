use reqwest::Client;
use scout::{ScoutProvider, providers::safebooru::SafebooruProvider};

#[tokio::test]
async fn t_sb() {
    let p = SafebooruProvider::new(Client::new());
    let items = p.fetch_content(5, 0).await.unwrap();
    dbg!(items);
}
