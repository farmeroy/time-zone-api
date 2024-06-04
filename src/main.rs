use chrono::Utc;
use chrono_tz::Tz;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tzf_rs::Finder;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
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
    pub address: Address,
    pub boundingbox: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub village: String,
    pub county: String,
    pub state: String,
    #[serde(rename = "ISO3166-2-lvl4")]
    pub iso3166_2_lvl4: String,
    pub country: String,
    pub country_code: String,
}

lazy_static! {
    static ref FINDER: Finder = Finder::new();
}

#[tokio::main]
async fn main() {
    let resp: Root = reqwest::Client::new()
        .get("https://nominatim.openstreetmap.org/search?addressdetails=1&q=comptche&format=jsonv2&limit=1")
        .header("User-Agent","Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0" )
        .header("Referer", "localhost")
        .send()
        .await
        .unwrap().json().await.unwrap();

    let (lat, lon) = (
        resp.get(0).unwrap().lat.parse().unwrap(),
        resp.get(0).unwrap().lon.parse().unwrap(),
    );
    println!("{}, {}", lat, lon);
    // get the time zone according to the geo coordinates
    let zone = FINDER.get_tz_name(lon, lat);
    print!("{:?}", zone);
    // parse it into a type that implements TimeZone
    let tz: Tz = zone.parse().unwrap();
    // create a new datetime with that type
    let now = Utc::now().with_timezone(&tz);
    println!("{}, {}, {:#?}", tz, now, resp);
}
