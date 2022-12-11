use sqlx::SqlitePool;

use crate::hangul;
use crate::model::{Card, Deck};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Database, sqlx::Error> {
        let pool = SqlitePool::connect("korean.db").await?;
        Ok(Database { pool })
    }

    pub async fn select_decks(&self) -> anyhow::Result<Vec<Deck>> {
        let results = sqlx::query!(
            r#"SELECT 
              id as "id!",
              name_display AS name,
              COUNT(cards.deck_id) AS card_count
            FROM decks
            JOIN cards ON cards.deck_id = decks.id"#
        )
        .map(|row| {
            let card_count = row.card_count.unwrap_or(0_i32);

            Deck {
                id: row.id,
                name: row.name,
                card_count,
            }
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn insert_deck(&self, name: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;

        let name_decomposed = hangul::decompose(name);

        sqlx::query!(
            r#"
            INSERT INTO decks (name_display, name_decomposed) VALUES (?1, ?2)
            "#,
            name,
            name_decomposed
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    pub async fn select_cards_for_deck(&self, deck_id: i32) -> anyhow::Result<Vec<Card>> {
        let results = sqlx::query_as!(
            Card,
            r#"SELECT 
                korean_display AS korean,
                english           
            FROM cards
            WHERE deck_id = ?1"#,
            deck_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
    pub async fn insert_card(
        &self,
        deck_id: i32,
        korean: &str,
        english: &str,
    ) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;

        let korean_decomposed = hangul::decompose(korean);

        sqlx::query!(
            r#"
            INSERT INTO cards
            (korean_display, korean_decomposed, english, deck_id) 
            VALUES 
            (?1, ?2, ?3, ?4)
            "#,
            korean,
            korean_decomposed,
            english,
            deck_id
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }
}
