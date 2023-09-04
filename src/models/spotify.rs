use std::collections::HashSet;
use chrono::{DateTime, Duration, Utc};
use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;
use crate::models::errors::ResponseError;

#[derive(poem_openapi::Object)]
pub struct TokenPayload {
    pub access_token: String,
    #[serde(with = "duration_second")]
    pub expires_in: Duration,
    pub expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
    #[serde(default, with = "space_separated_scopes", rename = "scope")]
    pub scopes: HashSet<String>,
}

#[derive(ApiResponse)]
pub enum SpotifyResponse {
    #[oai(status = 200)]
    SpotifyResponse(Json<String>),

    #[oai(status = 404)]
    NotFound(Json<ResponseError>),

    #[oai(status = 401)]
    BadRequest(Json<ResponseError>),
}