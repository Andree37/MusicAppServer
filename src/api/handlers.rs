use poem::web::{Data, Query};
use poem_openapi::OpenApi;
use poem_openapi::payload::{PlainText, Json};
use poem_openapi::types::Type;

use crate::models::genres::Genres;
use crate::models::song::SongResponse;
use crate::services::db::DB;
use crate::services::lastfm::LastFM;
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
    async fn get_spotify(&self, spotify: Data<&Spotify>, genre: Query<String>, limit: Query<u32>) -> PlainText<String> {
        if genre.0.len() == 0 {
            return PlainText("no genre provided".to_string());
        }
        if limit.0.is_none() {
            return PlainText("must specify limit".to_string());
        }
        let rec = spotify.0.get_recommendations(genre.0, limit.0).await.unwrap();
        return PlainText(format!("recommendations: {:?}", rec));
    }

    #[oai(path = "/spotify", method = "post")]
    async fn get_daily_songs(&self, spotify: Data<&Spotify>, lastfm: Data<&LastFM>, db: Data<&DB>, genre: Query<String>) -> SongResponse {
        let genre: Genres = genre.0.try_into().map_err(|e| poem::error::BadRequest(e))?;

        let spotify_track = spotify.0.generate_daily_song(&genre).await.map_err(|e| poem::error::NotFound(e))?;

        let artist_name = &spotify_track.artists.first().expect("no artist found").name;
        let song_name = &spotify_track.name;
        let link = spotify_track.external_urls.get("spotify").expect("no spotify link found");

        let lastfm_track = lastfm.0.get_details(artist_name, song_name).await.map_err(|e| poem::error::BadRequest(e))?;

        let description = &lastfm_track.track_description;
        let summary = &lastfm_track.track_summary;

        let song = db.0.save_song(song_name, artist_name, link, description, summary, &genre).await.map_err(|e| poem::error::InternalServerError(e))?;

        return Ok(Json(song));
    }
}