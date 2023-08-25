#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(poem_openapi::Enum)]
pub enum Genres{
    Pop,
    Rock,
    Metal,
}


