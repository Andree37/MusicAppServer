use rspotify::ClientError;
use rspotify::clients::BaseClient;
use rspotify::model::{Recommendations, SimplifiedTrack};
use crate::models::genres::Genres;

#[derive(Clone)]
pub struct Spotify {
    client: rspotify::ClientCredsSpotify,
}

impl Spotify {
    pub async fn new(client_id: String, secret: String) -> Result<Self, ClientError> {
        let creds = rspotify::Credentials {
            id: client_id,
            secret: Some(secret),
        };
        let client = rspotify::ClientCredsSpotify::new(creds);
        client.request_token().await?;

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

    pub async fn generate_daily_song(&self, genre: &Genres) -> Result<SimplifiedTrack, ClientError> {
        let genre: String = genre.into();
        let recommendations = self.get_recommendations(genre, 1).await?;
        let song = recommendations.tracks.first().expect("no song found").to_owned();

        return Ok(song);
    }
}