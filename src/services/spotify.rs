use anyhow::Result;
use rspotify::clients::BaseClient;
use rspotify::model::Recommendations;

#[derive(Clone)]
pub struct Spotify {
    client: rspotify::ClientCredsSpotify,
}

impl Spotify {
    pub async fn new(client_id: String, secret: String) -> Result<Self> {
        let creds = rspotify::Credentials {
            id: client_id,
            secret: Some(secret),
        };
        let client = rspotify::ClientCredsSpotify::new(creds);
        client.request_token().await?;

        return Ok(Self { client });
    }

    pub async fn get_recommendations(&self, genres: Vec<String>, limit: u32) -> Result<Recommendations> {
        let attributes = [
            rspotify::model::RecommendationsAttribute::MinEnergy(0.4),
            rspotify::model::RecommendationsAttribute::MinPopularity(50),
        ];

        let genres: Vec<&str> = genres.iter().map(AsRef::as_ref).collect();

        let recommendations = self.client.recommendations(
            attributes,
            Some([]),
            Some(genres),
            Some([]),
            None,
            Some(limit),
        ).await?;

        return Ok(recommendations);
    }
}