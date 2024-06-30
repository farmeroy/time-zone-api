use std::num::ParseFloatError;

use lambda_http::{
    aws_lambda_events::query_map::QueryMap, http::Method, run, service_fn, tower::ServiceBuilder,
    tracing, Error, Request, RequestExt, Response,
};
use lazy_static::lazy_static;
use tower_http::cors::{Any, CorsLayer};
use tzf_rs::Finder;

use serde_json::json;

struct Coordinates {
    lat: String,
    lon: String,
}

impl TryFrom<QueryMap> for Coordinates {
    type Error = &'static str;

    fn try_from(query: QueryMap) -> Result<Self, Self::Error> {
        match (query.first("lat"), query.first("lon")) {
            (Some(lat), Some(lon)) => Ok(Coordinates {
                lat: lat.to_owned(),
                lon: lon.to_owned(),
            }),
            (None, None) => Err("Missing query params"),
            (None, _) => Err("Missing 'lat' param"),
            (_, None) => Err("Missing 'lon' param"),
        }
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

async fn func(event: Request) -> Result<Response<String>, Error> {
    let coords = match Coordinates::try_from(event.query_string_parameters()) {
        Ok(c) => c,
        Err(e) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(json!({"error": e.to_string()}).to_string())?;
            return Ok(resp);
        }
    };

    let body = match get_tz_name(&coords.lat, &coords.lon) {
        Ok(timezone) => json!({ "timezone": timezone }).to_string(),
        Err(e) => json!({ "error": e.to_string() }).to_string(),
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .expect("Failed to render response");

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    // Define a layer to inject CORS headers
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_origin(Any);

    let handler = ServiceBuilder::new()
        // Add the CORS layer to the service
        .layer(cors_layer)
        .service(service_fn(func));

    run(handler).await?;
    Ok(())
}
