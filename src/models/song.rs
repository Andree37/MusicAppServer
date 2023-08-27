use poem_openapi::payload::Json;
use poem_openapi::ApiResponse;
use crate::models::errors::{ResponseError};

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


#[derive(ApiResponse)]
pub enum SongResponse {
    #[oai(status = 200)]
    Song(Json<Song>),

    #[oai(status = 404)]
    NotFound(Json<ResponseError>),
}

