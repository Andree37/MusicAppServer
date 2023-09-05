use chrono::{DateTime, Utc};

pub struct User {
    pub id: i32,
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
}

impl User {
    pub fn new(id: i32, access_token: String, expires_in: i32, expires_at: Option<DateTime<Utc>>, refresh_token: String) -> Self {
        Self {
            id,
            access_token,
            expires_in,
            expires_at,
            refresh_token: Some(refresh_token),
        }
    }
}