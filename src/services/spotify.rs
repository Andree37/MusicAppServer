use std::{env, fs};
use std::path::PathBuf;
use rspotify::{AuthCodePkceSpotify, AuthCodeSpotify, ClientError, Config, Credentials, OAuth, scopes, Token};
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{ArtistId, Recommendations, SearchType, SimplifiedAlbum, SimplifiedTrack};
use rspotify::model::SearchResult::Albums;
use rspotify::model::SearchType::Album;
use rspotify::model::Type::Artist;

use crate::models::genres::Genres;

#[derive(Clone)]
pub struct Spotify {
    pub client: AuthCodeSpotify,
}

const CACHE_PATH: &str = ".spotify_cache/";


impl Spotify {
    pub async fn from_code(code: String) -> Result<Self, ClientError> {
        let creds = Credentials::from_env().expect("RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET must be set in the environment");

        // Using every possible scope
        let scopes = scopes!(
        "user-read-private",
        "playlist-modify-public"
        );

        let oauth = OAuth::from_env(scopes).unwrap();

        let client = AuthCodeSpotify::new(creds, oauth);

        match client.request_token(&code).await {
            Ok(_) => {
                println!("Requested user token successfully");
            }
            Err(err) => {
                println!("Failed to get user token: {:?}", err);
            }
        }

        return Ok(Self { client });
    }

    pub fn from_token(token: Token) -> Self {
        let client = AuthCodeSpotify::from_token(token);

        return Self { client };
    }

    pub async fn get_recommendations(&self, genre: String, limit: u32) -> Result<Recommendations, ClientError> {
        let attributes = [
            rspotify::model::RecommendationsAttribute::MinEnergy(0.4),
            rspotify::model::RecommendationsAttribute::MinPopularity(50),
        ];

        let genre = genre.as_str();

        let recommendations = self.client.recommendations(
            attributes,
            Some([]),
            Some([genre]),
            Some([]),
            None,
            Some(limit),
        ).await?;

        return Ok(recommendations);
    }

    pub async fn search_track_details(&self, artist: &str, track: &str) -> Option<Vec<SimplifiedAlbum>> {
        let query = format!("{} {}", artist, track);
        let search_result = match self.client.search(&query, Album, None, None, Some(1), Some(0)).await {
            Ok(result) => result,
            Err(_) => return None,
        };

        return match search_result {
            Albums(album) => Some(album.items),
            _ => None,
        };
    }

    pub async fn generate_daily_song(&self, genre: &Genres) -> Option<SimplifiedTrack> {
        let genre: String = genre.into();
        let recommendations = match self.get_recommendations(genre, 1).await {
            Ok(rec) => rec,
            Err(_) => return None,
        };
        let song = match recommendations.tracks.first() {
            Some(song) => song.to_owned(),
            None => return None,
        };

        return Some(song);
    }
}