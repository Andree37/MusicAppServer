use chrono::{DateTime, NaiveDate, Utc};

use crate::models::genres::{Genre, GenreTypes};
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

    pub async fn save_song(&self, title: &str, artist: &str, link: &str, description: &str, overview: &str, genre: &GenreTypes, album_cover: &str) -> Result<Song, sqlx::Error> {
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

    pub async fn insert_user_genres(&self, user_id: i32, genre: Vec<GenreTypes>) -> Result<(), sqlx::Error> {
        let genres: Vec<String> = genre.iter().map(|g| g.into()).collect::<Vec<String>>();

        for genre in genres {
            sqlx::query!("INSERT INTO user_genres (user_id, genre_id) VALUES ($1, $2)", user_id, genre)
                .execute(&self.pool)
                .await?;
        }

        return Ok(());
    }

    pub async fn get_user_genres(&self, user_id: i32) -> Result<Vec<Genre>, sqlx::Error> {
        let genres = sqlx::query_as!(Genre, "SELECT name FROM user_genres ug LEFT JOIN genres g ON ug.genre_id = g.name WHERE user_id = $1", user_id)
            .fetch_all(&self.pool)
            .await?;
        return Ok(genres);
    }

    pub async fn update_user_token(&self, user_id: i32, access_token: &str, expires_in: i32, expires_at: &DateTime<Utc>, refresh_token: &String) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE users SET access_token = $1, expires_in = $2, expires_at = $3, refresh_token = $4 WHERE id = $5", access_token, expires_in, expires_at, refresh_token, user_id)
            .execute(&self.pool)
            .await?;
        return Ok(());
    }
}