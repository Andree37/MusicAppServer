use crate::models::genres::Genres;
use crate::models::song::Song;


#[derive(Clone)]
pub struct DB {
    pool: sqlx::PgPool,
}

impl DB {
    pub async fn new(database_url: String) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(&database_url).await?;
        return Ok(Self { pool });
    }

    pub async fn select_all_songs(&self) -> Result<Vec<Song>, sqlx::Error> {
        let songs = sqlx::query_as!(Song, "SELECT id, title, artist, link, description, overview, created_at, genre as \"genre: _\" FROM songs")
            .fetch_all(&self.pool)
            .await?;
        return Ok(songs);
    }

    pub async fn save_song(&self, title: &str, artist: &str, link: &str, description: &str, overview: &str, genre: &Genres) -> Result<Song, sqlx::Error> {
        let genre: String = genre.into();

        let song = sqlx::query_as!(Song, "INSERT INTO songs (title, artist, link, description, overview, genre) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, title, artist, link, description, overview, created_at, genre", title, artist, link, description, overview, genre)
            .fetch_one(&self.pool)
            .await?;
        return Ok(song);
    }
}