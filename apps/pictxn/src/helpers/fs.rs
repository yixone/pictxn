use std::path::Path;

pub async fn create_all_parents_if_exists(path: &Path) -> std::io::Result<()> {
    if let Some(p) = path.parent() {
        tokio::fs::create_dir_all(p).await?;
    }

    Ok(())
}
