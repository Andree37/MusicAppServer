use poem::web::Data;
use poem_openapi::{OpenApi, param::Query, payload::PlainText};
use poem_openapi::types::Type;

use crate::services::spotify::Spotify;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/spotify", method = "get")]
    async fn get_spotify(&self, spotify: Data<&Spotify>, genres: Query<Vec<String>>, limit: Query<u32>) -> PlainText<String> {
        if genres.0.len() == 0 {
            return PlainText("no genres provided".to_string());
        }
        if limit.0.is_none() {
            return PlainText("must specify limit".to_string());
        }
        let rec = spotify.0.get_recommendations(genres.0, limit.0).await.unwrap();
        return PlainText(format!("recommendations: {:?}", rec));
    }
}