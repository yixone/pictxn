use crate::{
    database::sqlite::db::SqliteDatabase,
    domains::cards::{id::CardId, model::Card, ops::AbstractCards},
    result::Result,
};

#[async_trait::async_trait]
impl AbstractCards for SqliteDatabase {
    /// Insert a new card into the database
    async fn insert_card(&self, card: &Card) -> Result<()> {
        sqlx::query(
            "
            INSERT INTO cards (
                id, file_id, created, title, description
            )
            VALUES (
                ?, ?, ?, ?, ?
            )
            ",
        )
        .bind(card.id)
        .bind(card.file_id)
        .bind(card.created)
        .bind(&card.title)
        .bind(&card.description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a card from the database by ID
    async fn get_card(&self, id: CardId) -> Result<Option<Card>> {
        let card = sqlx::query_as(
            "
            SELECT 
                id, file_id, created, title, description
            FROM cards
            WHERE id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(card)
    }

    /// Delete a card from the database by ID
    async fn remove_card(&self, id: CardId) -> Result<()> {
        sqlx::query(
            "
            DELETE FROM cards
            WHERE id = ?
            ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
