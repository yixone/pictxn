use std::path::Path;

pub async fn create_all_parents(path: &Path) -> std::io::Result<()> {
    if let Some(p) = path.parent() {
        tokio::fs::create_dir_all(p).await?;
    }

    Ok(())
}

pub fn create_all_parents_sync(path: &Path) -> std::io::Result<()> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }

    Ok(())
}
