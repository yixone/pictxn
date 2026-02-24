use std::sync::Arc;

use tracing::info;

use crate::tasks::task::BackgroundTask;

#[derive(Default)]
pub struct BackgroundTaskHost {
    tasks: Vec<Arc<dyn BackgroundTask + Send + Sync + 'static>>,
}

impl BackgroundTaskHost {
    pub fn new() -> Self {
        BackgroundTaskHost::default()
    }

    pub fn with_task<T>(mut self, task: T) -> Self
    where
        T: BackgroundTask + Send + Sync + 'static,
    {
        self.tasks.push(Arc::new(task));
        self
    }

    pub fn run(&self) {
        info!(tasks_count = self.tasks.len(), "Spawning background tasks");

        for task in &self.tasks {
            let task = task.clone();
            tokio::spawn(async move {
                task.run().await;
            });
        }
    }
}
