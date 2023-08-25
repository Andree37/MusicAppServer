pub struct DetailResponse {
    pub track_info: String,
    pub track_summary: String,
    pub track_description: String,
}

#[derive(Clone)]
pub struct LastFM {
    key: String,
}

impl LastFM {
    pub async fn new(key: String) -> Result<LastFM, reqwest::Error> {
        return Ok(Self { key });
    }

    pub async fn get_details(&self, artist_name: &str, track_name: &str) -> Result<DetailResponse, reqwest::Error> {
        let url = format!("http://ws.audioscrobbler.com/2.0/?method=track.getInfo&api_key={}&artist={}&track={}&format=json", &self.key, artist_name, track_name);
        let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

        let track_info = response["track"]["name"].as_str().unwrap_or("Track not found.").to_owned();
        let track_summary = response["track"]["wiki"]["summary"].as_str().unwrap_or("Summary not found.").to_owned();
        let track_description = response["track"]["wiki"]["content"].as_str().unwrap_or("Description not found.").to_owned();

        let detail_response = DetailResponse {
            track_info,
            track_summary,
            track_description,
        };

        return Ok(detail_response);
    }
}