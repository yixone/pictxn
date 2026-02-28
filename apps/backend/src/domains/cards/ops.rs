use crate::{
    domains::cards::{id::CardId, model::Card},
    result::Result,
};

#[async_trait::async_trait]
pub trait AbstractCards: Send + Sync {
    /// Insert a new card into the database
    async fn insert_card(&self, card: &Card) -> Result<()>;

    /// Get a card from the database by ID
    async fn get_card(&self, id: CardId) -> Result<Option<Card>>;

    /// Delete a card from the database by ID
    async fn remove_card(&self, id: CardId) -> Result<()>;
}
