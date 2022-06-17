mod key;
/*
pub fn key() -> &'static str {
  return "API_KEY";
}
*/
use reqwest::Result;
/*
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    location_epoch: i64,
    location_time: String,
}

#[derive(Deserialize)]
struct Current {
    last_updated_epoch: i64,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
}
*/

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = key::key();
    let request = format!(
        "http://api.weatherapi.com/v1/current.json?key={}{}",
        api_key, "&q=Warsaw"
    );
    let response = reqwest::get(&request).await?;
    println!("Status: {}", response.status());
    println!("Header:\n{:#?}", response.headers());
    let body = response.text().await?.to_string();
    println!("Body:\n{}", body);
    //body.to_string();
    Ok(())
}
