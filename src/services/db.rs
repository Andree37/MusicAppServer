use anyhow::Result;

use crate::models::song::Song;

#[derive(Clone)]
pub struct DB {
    pool: sqlx::PgPool,
}

impl DB {
    pub async fn new(database_url: String) -> Result<Self> {
        let pool = sqlx::PgPool::connect(&database_url).await?;
        return Ok(Self { pool });
    }

    pub async fn select_all_songs(&self) -> Result<Vec<Song>> {
        let songs = sqlx::query_as!(Song, "SELECT id, title, artist, rating, description, overview, created_at, genre as \"genre: _\" FROM songs")
            .fetch_all(&self.pool)
            .await?;
        return Ok(songs);
    }
}