use poem_openapi::payload::Json;
use poem::Result;

use crate::models::genres::Genres;

#[derive(poem_openapi::Object)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub genre: Genres,
    pub link: String,
    pub description: Option<String>,
    pub overview: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub type SongResponse = Result<Json<Song>>;