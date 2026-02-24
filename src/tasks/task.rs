#[async_trait::async_trait]
pub trait BackgroundTask {
    async fn run(&self);
}
