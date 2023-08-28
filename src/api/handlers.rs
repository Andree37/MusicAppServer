use chrono::NaiveDate;
use poem::Result;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Query;
use poem_openapi::payload::{Json, PlainText};

use crate::models::errors::ResponseError;
use crate::models::genres::{GenrePayload, Genres};
use crate::models::song::{SongResponse, SongsResponse};
use crate::services::db::DB;
use crate::services::lastfm::LastFM;
use crate::services::spotify::Spotify;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/songs", method = "post")]
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

        let albums = match spotify.search_track_details(artist_name, song_name).await {
            Some(albums) => match albums.first() {
                Some(album) => album.to_owned(),
                None => return Ok(SongResponse::NotFound(Json(ResponseError { message: "no album found 1".to_string() }))),
            }
            None => return Ok(SongResponse::NotFound(Json(ResponseError { message: "no album found 2".to_string() }))),
        };

        let album_cover = match albums.images.first() {
            Some(image) => &image.url,
            None => return Ok(SongResponse::NotFound(Json(ResponseError { message: "no album cover found 3".to_string() }))),
        };


        let lastfm_track = lastfm.0.get_details(artist_name, song_name).await.map_err(|e| poem::error::BadRequest(e))?;

        let description = &lastfm_track.track_description;
        let summary = &lastfm_track.track_summary;

        let song = db.0.save_song(song_name, artist_name, link, description, summary, &genre, album_cover).await.map_err(|e| poem::error::InternalServerError(e))?;

        return Ok(SongResponse::Song(Json(song)));
    }

    #[oai(path = "/songs", method = "get")]
    async fn get_songs(&self, day: Query<Option<String>>, db: Data<&DB>) -> Result<SongsResponse> {
        return match day.0 {
            Some(day) => {
                let day = NaiveDate::parse_from_str(&day, "%Y-%m-%d")
                    .map_err(|e| SongsResponse::BadRequest(Json(ResponseError { message: e.to_string() })))?;
                let songs = db.0.get_daily_songs(day)
                    .await.map_err(|e| SongsResponse::BadRequest(Json(ResponseError { message: e.to_string() })))?;
                Ok(SongsResponse::Song(Json(songs)))
            }
            None => Ok(SongsResponse::NotFound(Json(ResponseError { message: "no day provided".to_string() })))
        };
    }
}