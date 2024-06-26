use chrono_tz::Tz;
use lambda_runtime::service_fn;
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
    let func = service_fn(get_time_zone);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn get_time_zone(search: Query<SearchParams>) -> (StatusCode, Json<String>) {
    let zone = FINDER.get_tz_name(search.lon, search.lat);
    let tz: Tz = zone.parse().unwrap();
    let tz = tz.name().to_owned();
    (StatusCode::OK, Json(tz))
}
