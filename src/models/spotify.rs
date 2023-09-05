use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;

use crate::models::errors::ResponseError;

#[derive(poem_openapi::Object)]
pub struct CodePayload {
    pub code: String,
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