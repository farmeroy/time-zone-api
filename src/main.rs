use std::net::SocketAddr;

use axum::{extract::Query, routing::get, Json, Router};
use chrono::Utc;
use chrono_tz::Tz;
use lazy_static::lazy_static;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tzf_rs::Finder;

pub type Places = Vec<Place>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Place {
    pub place_id: i64,
    pub licence: String,
    pub osm_type: String,
    pub osm_id: i64,
    pub lat: String,
    pub lon: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub place_rank: i64,
    pub importance: f64,
    pub addresstype: String,
    pub name: String,
    pub display_name: String,
    pub boundingbox: Vec<String>,
    pub icon: Option<String>,
    pub namedetails: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceAndTime {
    pub place: Place,
    pub time_now: String,
    pub time_zone: String,
}

#[derive(Deserialize)]
struct SearchParams {
    place: String,
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
        .route("/search", get(search))
}

async fn check_health() -> (StatusCode, String) {
    (
        StatusCode::OK,
        String::from("Hello from the time zone finder!"),
    )
}

async fn search(search: Query<SearchParams>) -> (StatusCode, Json<PlaceAndTime>) {
    let query_string = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=jsonv2&limit=1",
        search.place
    );
    let resp: Places = reqwest::Client::new()
        .get(query_string)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0",
        )
        .header("Referer", "localhost")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let (lat, lon) = (
        resp.first().unwrap().lat.parse().unwrap(),
        resp.first().unwrap().lon.parse().unwrap(),
    );
    let zone = FINDER.get_tz_name(lon, lat);
    let tz: Tz = zone.parse().unwrap();
    // create a new datetime with that type
    let now = Utc::now().with_timezone(&tz);
    let place_and_time = PlaceAndTime {
        place: resp.first().unwrap().clone(),
        time_now: now.to_string(),
        time_zone: tz.to_string(),
    };
    (StatusCode::OK, Json(place_and_time))
}
