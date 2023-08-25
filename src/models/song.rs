use crate::models::genres::Genres;

#[derive(poem_openapi::Object)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub genre: Genres,
    pub rating: i32,
    pub description: Option<String>,
    pub overview: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}