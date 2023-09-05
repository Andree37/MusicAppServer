extern crate dotenv;

use std::env;

use poem::{EndpointExt, listener::TcpListener, Route};
use poem::session::{CookieConfig, CookieSession};
use poem_openapi::OpenApiService;

use crate::services::db::DB;
use crate::services::lastfm::LastFM;

mod api;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let db = DB::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await?;

    let lastfm = LastFM::new(env::var("LAST_FM_KEY").expect("LAST_FM_KEY must be set")).await?;

    let api_service =
        OpenApiService::new(api::handlers::Api, "Hello World", "1.0").server("http://localhost:3000/api");

    let ui = api_service.openapi_explorer();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/ui", ui)
        .with(CookieSession::new(CookieConfig::default().secure(false)))
        .data(lastfm)
        .data(db);


    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .map_err(|e| e.into())
}