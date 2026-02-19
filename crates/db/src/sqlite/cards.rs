use models::cards::{Card, CardId};

use crate::{ops::cards::AbstractCards, sqlite::SqliteDatabaseInner};

#[async_trait::async_trait]
impl AbstractCards for SqliteDatabaseInner {
    /// Insert a new card into the database
    async fn insert_card(&self, card: &Card) -> Result<(), sqlx::Error> {
        todo!()
    }

    /// Get a card from the database by ID
    async fn get_card(&self, id: &CardId) -> Result<Option<Card>, sqlx::Error> {
        todo!()
    }

    /// Delete a card from the database by ID
    async fn remove_card(&self, id: &CardId) -> Result<(), sqlx::Error> {
        todo!()
    }
}
