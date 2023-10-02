use poem::session::Session;
use rspotify::{AuthCodeSpotify, ClientError, Credentials, OAuth, scopes, Token};
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{Recommendations, SimplifiedAlbum, SimplifiedTrack};
use rspotify::model::SearchResult::Albums;
use rspotify::model::SearchType::Album;

use crate::models::genres::GenreTypes;
use crate::token::token::write_token;

#[derive(Clone)]
pub struct Spotify {
    pub client: AuthCodeSpotify,
}


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

    pub async fn from_token(token: Token, session: &Session) -> Result<Self, ClientError> {
        let client = AuthCodeSpotify::from_token(token.clone());

        match token.expires_at {
            Some(expires_at) => {
                let now = chrono::Utc::now();
                if now > expires_at {
                    match client.refresh_token().await {
                        Ok(_) => {
                            match write_token(token, &session) {
                                Ok(_) => {
                                    println!("Refreshed token successfully");
                                }
                                Err(err) => {
                                    println!("Failed to write token: {:?}", err)
                                }
                            }
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
            }
            None => {}
        }

        return Ok(Self { client });
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

    pub async fn generate_daily_song(&self, genre: &GenreTypes) -> Option<SimplifiedTrack> {
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