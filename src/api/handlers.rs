use poem::web::{Data, Query};
use poem_openapi::OpenApi;
use poem_openapi::payload::{PlainText, Json};
use poem_openapi::types::Type;
use poem::Result;
use crate::models::errors::ResponseError;

use crate::models::genres::{GenrePayload, Genres};
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
        let rec = match spotify.0.get_recommendations(genre.0, limit.0).await {
            Ok(rec) => rec,
            Err(_) => return PlainText("error".to_string()),
        };
        return PlainText(format!("recommendations: {:?}", rec));
    }

    #[oai(path = "/spotify", method = "post")]
    async fn get_daily_songs(&self, spotify: Data<&Spotify>, genre: Json<GenrePayload>, lastfm: Data<&LastFM>, db: Data<&DB>) -> Result<SongResponse> {
        let genre: Genres = genre.0.genre.into();

        if genre == Genres::Unknown {
            return Ok(SongResponse::NotFound(Json(ResponseError { message: "invalid genre".to_string() })));
        }

        let spotify_track = match spotify.0.generate_daily_song(&genre).await {
            Some(track) => track,
            None => return Ok(SongResponse::NotFound(Json(ResponseError { message: "no track found".to_string() }))),
        };

        let artist_name = match &spotify_track.artists.first() {
            Some(artist) => &artist.name,
            None => return Ok(SongResponse::NotFound(Json(ResponseError { message: "no artist found".to_string() }))),
        };

        let song_name = &spotify_track.name;
        let link = match spotify_track.external_urls.get("spotify") {
            Some(link) => link,
            None => return Ok(SongResponse::NotFound(Json(ResponseError { message: "no link found".to_string() }))),
        };

        let lastfm_track = lastfm.0.get_details(artist_name, song_name).await.map_err(|e| poem::error::BadRequest(e))?;

        let description = &lastfm_track.track_description;
        let summary = &lastfm_track.track_summary;

        let song = db.0.save_song(song_name, artist_name, link, description, summary, &genre).await.map_err(|e| poem::error::InternalServerError(e))?;

        return Ok(SongResponse::Song(Json(song)));
    }
}