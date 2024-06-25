use std::net::SocketAddr;

use axum::{extract::Query, routing::get, Json, Router};
use chrono_tz::Tz;
use lazy_static::lazy_static;
use reqwest::StatusCode;
use serde::Deserialize;
use tzf_rs::Finder;

#[derive(Deserialize)]
struct SearchParams {
    lat: f64,
    lon: f64,
}

lazy_static! {
    static ref FINDER: Finder = Finder::new();
}

#[tokio::main]
async fn main() {
    let app = router().await;
    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn router() -> Router {
    Router::new()
        .route("/", get(check_health))
        .route("/tz", get(get_time_zone))
}

async fn check_health() -> (StatusCode, String) {
    (
        StatusCode::OK,
        String::from("Hello from the time zone finder!"),
    )
}

async fn get_time_zone(search: Query<SearchParams>) -> (StatusCode, Json<String>) {
    let zone = FINDER.get_tz_name(search.lon, search.lat);
    let tz: Tz = zone.parse().unwrap();
    let tz = tz.name().to_owned();
    (StatusCode::OK, Json(tz))
}
