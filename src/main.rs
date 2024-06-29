use std::num::ParseFloatError;

use chrono_tz::Tz;
use lambda_http::{run, service_fn, tracing, Error, IntoResponse, Request, RequestExt, Response};
use lazy_static::lazy_static;
use serde_json::json;
use tzf_rs::Finder;

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
    let lat_query: Option<&str> = match event.query_string_parameters_ref() {
        Some(params) => params.first("lat"),
        None => None,
    };
    let lon_query: Option<&str> = match event.query_string_parameters_ref() {
        Some(params) => params.first("lon"),
        None => None,
    };
    let body;
    if lat_query.is_some() && lon_query.is_some() {
        body = match get_tz_name(lat_query.unwrap(), lon_query.unwrap()) {
            Ok(res) => json!({
                "timezone": res
            })
            .to_string(),
            Err(e) => json!({
                "error": e.to_string()
            })
            .to_string(),
        }
    } else {
        body = json!({
            "error": "Missing query"
        })
        .to_string();
    };

    //let tz: Tz = zone.parse().unwrap();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
