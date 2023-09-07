use poem_openapi::{Enum, Object};

pub struct Genre {
    pub name: GenreTypes,
}

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Enum)]
#[derive(PartialEq)]
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
