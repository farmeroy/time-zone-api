use std::num::ParseFloatError;

use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, tracing, Error, IntoResponse, Request,
    RequestExt, Response,
};
use lazy_static::lazy_static;
use tzf_rs::Finder;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct Coordinates {
    lat: String,
    lon: String,
}

impl TryFrom<QueryMap> for Coordinates {
    type Error = &'static str;

    fn try_from(query: QueryMap) -> Result<Self, Self::Error> {
        let lat = query
            .first("lat")
            .ok_or("Missing 'lat' parameter")?
            .to_string();
        let lon = query
            .first("lon")
            .ok_or("Missing 'lon' parameter")?
            .to_string();
        Ok(Coordinates { lat, lon })
    }
}

lazy_static! {
    static ref FINDER: Finder = Finder::new();
}

fn get_tz_name(lat: &str, lon: &str) -> Result<&'static str, ParseFloatError> {
    let lat = lat.parse();
    let lon = lon.parse();
    match (lat, lon) {
        (Ok(lat), Ok(lon)) => {
            let tz = FINDER.get_tz_name(lon, lat);
            Ok(tz)
        }
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let coords: Coordinates = event
        .query_string_parameters()
        .try_into()
        .map_err(|_| "Missing or invalid query parameters")?;

    let body = match get_tz_name(&coords.lat, &coords.lon) {
        Ok(timezone) => json!({ "timezone": timezone }).to_string(),
        Err(e) => json!({ "error": e.to_string() }).to_string(),
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
