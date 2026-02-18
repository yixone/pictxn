use models::cards::{Card, CardId};

#[async_trait::async_trait]
pub trait AbstractCards: Send + Sync {
    /// Insert a new card into the database
    async fn insert_card(&self, card: &Card) -> Result<(), sqlx::Error>;

    /// Get a card from the database by ID
    async fn get_card(&self, id: &CardId) -> Result<Option<Card>, sqlx::Error>;

    /// Delete a card from the database by ID
    async fn remove_card(&self, id: &CardId) -> Result<(), sqlx::Error>;
}
