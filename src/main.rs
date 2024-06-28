use chrono_tz::Tz;
use lambda_http::{run, service_fn, tracing, Error, IntoResponse, Request, RequestExt, Response};
use serde_json::json;
use tzf_rs::Finder;

async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let lat: f64 = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("lat"))
        .unwrap()
        .to_owned()
        .parse()
        .unwrap();
    let lon: f64 = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("lon"))
        .unwrap()
        .to_owned()
        .parse()
        .unwrap();
    let finder = Finder::new();
    let zone = finder.get_tz_name(lon, lat);
    let tz: Tz = zone.parse().unwrap();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            json!({
                "timezone": tz.name()
            })
            .to_string(),
        )
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
