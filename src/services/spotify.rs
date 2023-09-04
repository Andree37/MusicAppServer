use rspotify::{AuthCodeSpotify, ClientError, OAuth, scopes, Token};
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{Recommendations, SimplifiedAlbum, SimplifiedTrack};
use rspotify::model::SearchResult::Albums;
use rspotify::model::SearchType::Album;

use crate::models::genres::Genres;

#[derive(Clone)]
pub struct Spotify {
    client: AuthCodeSpotify,
}

impl Spotify {
    pub fn new(token: &Token) -> Result<Self, ClientError> {
        let client = AuthCodeSpotify::from_token(token.clone());

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