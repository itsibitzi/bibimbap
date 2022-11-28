use sqlx::SqlitePool;

use crate::todo::EncryptedTodo;
use crate::user::User;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Database, sqlx::Error> {
        let pool = SqlitePool::connect("korean.db").await?;
        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        Ok(sqlx::migrate!().run(&self.pool).await?)
    }

    pub async fn select_decks(&self) -> anyhow::Result<Vec<Deck>> {
        let results = sqlx::query_as!(
            Deck,
            "SELECT 
              id,
              name_display AS name,
              COUNT(cards.*) AS card_count
            FROM decks
            JOIN cards ON cards.deck_id = decks.id",
            username
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn insert_deck(&self, name: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
            INSERT INTO decks (name) VALUES (?1)
            "#,
            name
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    pub async fn insert_card(
        &self,
        deck_id: i32,
        korean: &str,
        english: &str,
    ) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
            INSERT INTO cards
            (korean_display, korean_decomposed, english, deck_id) 
            VALUES 
            (?1, ?2, ?3, ?4)
            "#,
            korean,
            korean_display,
            english,
            deck_id
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }
}
