use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;
use rspotify::Token;
use crate::models::errors::ResponseError;

#[derive(poem_openapi::Object)]
pub struct TokenPayload {
    pub(crate) authorization_code: Token,
}

#[derive(ApiResponse)]
pub enum SpotifyResponse {
    #[oai(status = 200)]
    SpotifyResponse(Json<Token>),

    #[oai(status = 404)]
    NotFound(Json<ResponseError>),

    #[oai(status = 401)]
    BadRequest(Json<ResponseError>),
}