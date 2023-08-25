use std::fmt::Error;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(poem_openapi::Enum)]
pub enum Genres {
    Unknown,
    Pop,
    Rock,
    Metal,
}

impl TryFrom<String> for Genres {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        match value {
            s if s.eq_ignore_ascii_case("pop") => Ok(Genres::Pop),
            s if s.eq_ignore_ascii_case("rock") => Ok(Genres::Rock),
            s if s.eq_ignore_ascii_case("metal") => Ok(Genres::Metal),
            _ => Ok(Genres::Unknown),
        }
    }
}

impl TryFrom<Genres> for String {
    type Error = Error;

    fn try_from(value: Genres) -> Result<Self, Error> {
        let e = match value {
            Genres::Unknown => "unknown".to_string(),
            Genres::Pop => "pop".to_string(),
            Genres::Rock => "rock".to_string(),
            Genres::Metal => "metal".to_string(),
        };
        return Ok(e);
    }
}

impl TryFrom<&Genres> for String {
    type Error = Error;

    fn try_from(value: &Genres) -> Result<Self, Error> {
        let e = match value {
            Genres::Unknown => "unknown".to_string(),
            Genres::Pop => "pop".to_string(),
            Genres::Rock => "rock".to_string(),
            Genres::Metal => "metal".to_string(),
        };
        return Ok(e);
    }
}
