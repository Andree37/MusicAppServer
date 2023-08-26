use poem_openapi::{Enum, Object};

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Enum)]
pub enum Genres {
    Unknown,
    Pop,
    Rock,
    Metal,
}

#[derive(Object)]
pub struct GenrePayload {
    pub genre: String,
}

impl From<String> for Genres {
    fn from(value: String) -> Self {
        match value {
            s if s.eq_ignore_ascii_case("pop") => Genres::Pop,
            s if s.eq_ignore_ascii_case("rock") => Genres::Rock,
            s if s.eq_ignore_ascii_case("metal") => Genres::Metal,
            _ => Genres::Unknown,
        }
    }
}

impl From<Genres> for String {
    fn from(value: Genres) -> Self {
        return match value {
            Genres::Unknown => "unknown".to_string(),
            Genres::Pop => "pop".to_string(),
            Genres::Rock => "rock".to_string(),
            Genres::Metal => "metal".to_string(),
        };
    }
}

impl From<&Genres> for String {
    fn from(value: &Genres) -> Self {
        return match value {
            Genres::Unknown => "unknown".to_string(),
            Genres::Pop => "pop".to_string(),
            Genres::Rock => "rock".to_string(),
            Genres::Metal => "metal".to_string(),
        };
    }
}
