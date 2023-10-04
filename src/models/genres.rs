use poem_openapi::{ApiResponse, Enum, Object};
use poem_openapi::payload::Json;
use crate::models::errors::ResponseError;

pub struct Genre {
    pub name: GenreTypes,
}

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Enum)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum GenreTypes {
    Unknown,
    Pop,
    Rock,
    Metal,
}

#[derive(Object)]
pub struct GenrePayload {
    pub genre: String,
}

#[derive(Object)]
pub struct GenresPayload {
    pub genres: Vec<String>,
}

impl From<String> for GenreTypes {
    fn from(value: String) -> Self {
        match value {
            s if s.eq_ignore_ascii_case("pop") => GenreTypes::Pop,
            s if s.eq_ignore_ascii_case("rock") => GenreTypes::Rock,
            s if s.eq_ignore_ascii_case("metal") => GenreTypes::Metal,
            _ => GenreTypes::Unknown,
        }
    }
}

impl From<GenreTypes> for String {
    fn from(value: GenreTypes) -> Self {
        return match value {
            GenreTypes::Unknown => "unknown".to_string(),
            GenreTypes::Pop => "pop".to_string(),
            GenreTypes::Rock => "rock".to_string(),
            GenreTypes::Metal => "metal".to_string(),
        };
    }
}

impl From<&GenreTypes> for String {
    fn from(value: &GenreTypes) -> Self {
        return match value {
            GenreTypes::Unknown => "unknown".to_string(),
            GenreTypes::Pop => "pop".to_string(),
            GenreTypes::Rock => "rock".to_string(),
            GenreTypes::Metal => "metal".to_string(),
        };
    }
}

#[derive(ApiResponse)]
pub enum GenreResponse {
    #[oai(status = 200)]
    GenreResponse(Json<Vec<String>>),

    #[oai(status = 404)]
    NotFound(Json<ResponseError>),

    #[oai(status = 401)]
    BadRequest(Json<ResponseError>),
}