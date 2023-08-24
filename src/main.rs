extern crate dotenv;

use std::env;

use anyhow::Result;
use poem::{listener::TcpListener, Route};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};
use rspotify::model::{AlbumId, ArtistId, RecommendationsAttribute, TrackId};
use rspotify::prelude::BaseClient;
struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(poem_openapi::Enum)]
enum Genres{
    Pop,
    Rock,
    Metal,
}

#[derive(poem_openapi::Object)]
struct Song {
    id : i32,
    title : String,
    artist : String,
    genre : Genres,
    rating : i32,
    description : Option<String>,
    overview : Option<String>,
    created_at : chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load variables from .env file
    dotenv::dotenv().ok();

    let pool = sqlx::PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await?;

    let row = sqlx::query_as!(Song, "SELECT id, title, artist, rating, description, overview, created_at, genre as \"genre: _\" FROM songs")
        .fetch_one(&pool)
        .await?;

    println!("Value from database: {:?}", row.artist);


    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");

    let ui = api_service.openapi_explorer();
    let app = Route::new().nest("/api", api_service).nest("/ui", ui);

    // remove from here after

    let creds = rspotify::Credentials {
        id: env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set"),
        secret: Some(env::var("SPOTIFY_SECRET").expect("SPOTIFY_SECRET must be set")),
    };

    let spotify = rspotify::ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();


    let attributes = [
        RecommendationsAttribute::MinEnergy(0.4),
        RecommendationsAttribute::MinPopularity(50),
    ];
    let seed_artists = [ArtistId::from_id("4NHQUGzhtTLFvgF5SZesLK").unwrap()];
    let seed_tracks = [TrackId::from_id("0c6xIDDpzE81m2q797ordA").unwrap()];

    let genres = ["Country"];

    let recommendations = spotify.recommendations(
        attributes,
        Some(seed_artists),
        Some(genres),
        Some(seed_tracks),
        None,
        Some(10),
    ).await?;

    println!("Response: {:#?}", recommendations);

    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(birdy_uri).await;

    // println!("Response: {albums:#?}");

    let artist_name = "Coldplay";
    let track_name = "Yellow";  // Replace with your desired track name
    let api_key = env::var("LAST_FM_KEY").expect("LAST_FM_KEY must be set");

    let url = format!("http://ws.audioscrobbler.com/2.0/?method=track.getInfo&api_key={}&artist={}&track={}&format=json", api_key, artist_name, track_name);
    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

    let track_info = response["track"]["name"].as_str().unwrap_or("Track not found.");
    let track_summary = response["track"]["wiki"]["summary"].as_str().unwrap_or("Summary not found.");
    let track_description = response["track"]["wiki"]["content"].as_str().unwrap_or("Description not found.");

    println!("Track Info: {}", track_info);
    println!("Summary: {}", track_summary);
    println!("Description: {}", track_description);


    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .map_err(|e| e.into())
}