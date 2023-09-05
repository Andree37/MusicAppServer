use chrono::{DateTime, NaiveDate, Utc};

use crate::models::genres::Genres;
use crate::models::song::Song;
use crate::models::user::User;

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
        let songs = sqlx::query_as!(Song, "SELECT id, title, artist, link, description, overview, created_at, genre, album_cover FROM songs")
            .fetch_all(&self.pool)
            .await?;
        return Ok(songs);
    }

    pub async fn save_song(&self, title: &str, artist: &str, link: &str, description: &str, overview: &str, genre: &Genres, album_cover: &str) -> Result<Song, sqlx::Error> {
        let genre: String = genre.into();

        let song = sqlx::query_as!(Song, "INSERT INTO songs (title, artist, link, description, overview, genre, album_cover) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, title, artist, link, description, overview, created_at, genre, album_cover", title, artist, link, description, overview, genre, album_cover)
            .fetch_one(&self.pool)
            .await?;
        return Ok(song);
    }

    pub async fn get_daily_songs(&self, day: NaiveDate) -> Result<Vec<Song>, sqlx::Error> {
        let songs = sqlx::query_as!(Song, "SELECT id, title, artist, link, description, overview, created_at, genre, album_cover FROM songs WHERE created_at::date >= $1 and created_at::date < $1 + interval '1 day'", day)
            .fetch_all(&self.pool)
            .await?;
        return Ok(songs);
    }

    pub async fn insert_user(&self, access_token: &str, expires_in: i32, expires_at: Option<DateTime<Utc>>, refresh_token: Option<String>) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "INSERT INTO users (access_token, expires_in, expires_at, refresh_token) VALUES ($1, $2, $3, $4) RETURNING id,access_token, expires_in, expires_at, refresh_token", access_token, expires_in, expires_at, refresh_token)
            .fetch_one(&self.pool)
            .await?;
        return Ok(user);
    }
}