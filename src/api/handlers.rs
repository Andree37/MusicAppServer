use chrono::NaiveDate;
use poem::Result;
use poem::session::Session;
use poem::web::{Data};
use poem_openapi::OpenApi;
use poem_openapi::param::Query;
use poem_openapi::payload::{Json};

use crate::models::errors::ResponseError;
use crate::models::genres::{GenreResponse, GenresPayload};
use crate::models::song::{Song, SongsResponse};
use crate::models::spotify::{CodePayload, SpotifyResponse};
use crate::services::db::DB;
use crate::services::lastfm::LastFM;
use crate::services::spotify::Spotify;
use crate::token::token::{read_token, write_token};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/spotify/exchange", method = "post")]
    async fn exchange_token(&self, code: Json<CodePayload>, db: Data<&DB>, session: &Session) -> Result<SpotifyResponse> {
        let spotify = Spotify::from_code(code.0.code).await.map_err(|e| poem::error::BadRequest(e))?;
        let token = match spotify.client.token.clone().lock().await {
            Ok(token) => match token.clone() {
                Some(token) => token,
                None => return Ok(SpotifyResponse::NotFound(Json(ResponseError { message: "could not get the token from the mutex guard".to_string() }))),
            },
            Err(_) => return Ok(SpotifyResponse::NotFound(Json(ResponseError { message: "could not get the mutex lock".to_string() }))),
        };

        match write_token(token.clone(), session) {
            Ok(_) => {}
            Err(e) => {
                return Ok(SpotifyResponse::NotFound(Json(ResponseError { message: e.to_string() })));
            }
        }

        let expires_in = token.expires_in.num_seconds();
        let user = db.0.insert_user(&token.access_token, expires_in as i32, token.expires_at, token.refresh_token).await.map_err(|e| poem::error::BadRequest(e))?;
        session.set("user_id", user.id);

        Ok(SpotifyResponse::SpotifyResponse(Json("success".to_string())))
    }

    #[oai(path = "/songs", method = "post")]
    async fn get_daily_songs(&self, lastfm: Data<&LastFM>, db: Data<&DB>, session: &Session) -> Result<SongsResponse> {
        let token = match read_token(session) {
            Ok(token) => token,
            Err(e) => return Ok(SongsResponse::NotFound(Json(ResponseError { message: e.to_string() }))),
        };
        let spotify = Spotify::from_token(token.clone(), session).await.map_err(|e| poem::error::NotAcceptable(e))?;

        // update the token on the database
        let user_id = match session.get("user_id") {
            Some(user_id) => user_id,
            None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no user id found".to_string() }))),
        };

        db.0.update_user_token(user_id, &token.access_token, token.expires_in.num_seconds() as i32, &token.expires_at.unwrap(), &token.refresh_token.unwrap()).await.map_err(|e| poem::error::BadRequest(e))?;

        let previous_user_songs = db.0.get_all_songs_from_user(user_id).await.map_err(|e| poem::error::BadRequest(e))?;
        let genres = db.0.get_user_genres(1).await.map_err(|e| poem::error::BadRequest(e))?;
        let mut songs: Vec<Song> = vec![];
        for genre in genres {
            let genre = genre.name.into();
            let spotify_track = match spotify.generate_daily_song(&genre).await {
                Some(track) => track,
                None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no track found".to_string() }))),
            };

            // This can make an infinite loop, but I am okay with that for now ::)
            let mut song_exists = previous_user_songs.iter().any(|song| song.title == spotify_track.name && song.artist == spotify_track.artists.first().unwrap().name);
            while song_exists {
                let spotify_track = match spotify.generate_daily_song(&genre).await {
                    Some(track) => track,
                    None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no track found".to_string() }))),
                };
                song_exists = previous_user_songs.iter().any(|song| song.title == spotify_track.name && song.artist == spotify_track.artists.first().unwrap().name);
            }

            let artist_name = match &spotify_track.artists.first() {
                Some(artist) => &artist.name,
                None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no artist found".to_string() }))),
            };

            let song_name = &spotify_track.name;
            let link = match spotify_track.external_urls.get("spotify") {
                Some(link) => link,
                None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no link found".to_string() }))),
            };

            let albums = match spotify.search_track_details(artist_name, song_name).await {
                Some(albums) => match albums.first() {
                    Some(album) => album.to_owned(),
                    None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no album found 1".to_string() }))),
                }
                None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no album found 2".to_string() }))),
            };

            let album_cover = match albums.images.first() {
                Some(image) => &image.url,
                None => return Ok(SongsResponse::NotFound(Json(ResponseError { message: "no album cover found 3".to_string() }))),
            };

            let lastfm_track = match lastfm.0.get_details(artist_name, song_name).await {
                Ok(track) => track,
                Err(e) => return Ok(SongsResponse::NotFound(Json(ResponseError { message: e.to_string() }))),
            };

            let description = &lastfm_track.track_description;
            let summary = &lastfm_track.track_summary;

            let song = match db.0.save_song(song_name, artist_name, link, description, summary, &genre, album_cover).await {
                Ok(song) => song,
                Err(e) => return Ok(SongsResponse::NotFound(Json(ResponseError { message: e.to_string() }))),
            };
            songs.push(song);
        }
        return Ok(SongsResponse::Song(Json(songs)));
    }

    #[oai(path = "/songs", method = "get")]
    async fn get_songs(&self, day: Query<Option<String>>, db: Data<&DB>, session: &Session) -> Result<SongsResponse> {
        return match day.0 {
            Some(day) => {
                let day = NaiveDate::parse_from_str(&day, "%Y-%m-%d")
                    .map_err(|e| SongsResponse::BadRequest(Json(ResponseError { message: e.to_string() })))?;
                let user_id = session.get("user_id").ok_or(SongsResponse::NotFound(Json(ResponseError { message: "no user id found".to_string() })))?;
                let songs = db.0.get_daily_songs(&day, user_id)
                    .await.map_err(|e| SongsResponse::BadRequest(Json(ResponseError { message: e.to_string() })))?;
                Ok(SongsResponse::Song(Json(songs)))
            }
            None => Ok(SongsResponse::NotFound(Json(ResponseError { message: "no day provided".to_string() })))
        };
    }

    #[oai(path = "/genres", method = "post")]
    async fn save_genres(&self, genres: Json<GenresPayload>, db: Data<&DB>, session: &Session) -> Result<SpotifyResponse> {
        let user_id = session.get("user_id").ok_or(SpotifyResponse::NotFound(Json(ResponseError { message: "no user id found".to_string() })))?;
        let genres = genres.0.genres.iter().map(|genre| genre.clone().into()).collect();
        let result = match db.0.insert_user_genres(user_id, genres).await {
            Ok(_) => "success".to_string(),
            Err(e) => return Ok(SpotifyResponse::NotFound(Json(ResponseError { message: e.to_string() }))),
        };
        return Ok(SpotifyResponse::SpotifyResponse(Json(result)));
    }

    #[oai(path = "/genres", method = "get")]
    async fn get_genres(&self, db: Data<&DB>, session: &Session) -> Result<GenreResponse> {
        let user_id = session.get("user_id").ok_or(GenreResponse::NotFound(Json(ResponseError { message: "no user id found".to_string() })))?;
        let genres = db.0.get_user_genres(user_id).await.map_err(|e| GenreResponse::NotFound(Json(ResponseError { message: e.to_string() })))?;
        let genres = genres.iter().map(|genre| genre.name.into()).collect();

        return Ok(GenreResponse::GenreResponse(Json(genres)));
    }

    #[oai(path = "/playlist", method = "get")]
    async fn generate_playlist(&self, db: Data<&DB>, session: &Session) -> Result<SpotifyResponse> {
        let user_id = session.get("user_id").ok_or(SpotifyResponse::NotFound(Json(ResponseError { message: "no user id found".to_string() })))?;
        let day: NaiveDate = chrono::Utc::now().naive_utc().date();
        let songs = db.0.get_daily_songs(&day, user_id).await.map_err(|e| SpotifyResponse::NotFound(Json(ResponseError { message: e.to_string() })))?;
        let token = match read_token(session) {
            Ok(token) => token,
            Err(e) => return Ok(SpotifyResponse::NotFound(Json(ResponseError { message: e.to_string() }))),
        };

        let spotify = Spotify::from_token(token.clone(), session).await.map_err(|e| poem::error::NotAcceptable(e))?;
        let playlist_id = spotify.create_playlist(&day).await.map_err(|e| poem::error::NotAcceptable(e))?;
        let mut uris: Vec<String> = vec![];
        for song in songs {
            uris.push(song.link);
        }
        spotify.add_songs_to_playlist(playlist_id, uris).await.map_err(|e| poem::error::NotAcceptable(e))?;
        Ok(SpotifyResponse::SpotifyResponse(Json("success".to_string())))
    }
}