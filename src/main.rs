mod key;
/*
pub fn key() -> &'static str {
  return "API_KEY";
}
*/
mod json_converter;
use reqwest::{Result};
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: u64,
    localtime: String,
}

#[derive(Serialize, Deserialize)]
struct Condition {
    text: String,
    icon: String,
    code: u16,
}

#[derive(Serialize, Deserialize)]
struct Current {
    last_updated_epoch: u64,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: i8,
    #[serde(flatten)]
    conditions: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u8,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: u8,
    cloud: u8,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}



#[derive(Serialize, Deserialize,)]
struct Data {
    #[serde(flatten)]
    location: Location,
    #[serde(flatten)]
    current: Current,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = key::key();
    let request = format!(
        "http://api.weatherapi.com/v1/current.json?key={}{}",
        api_key, "&q=Warsaw"
    );
    let response = reqwest::get(&request).await?;
    let body = response.text().await?;
    let parsed: Data = serde_json::from_str(&body).unwrap();
    /*
    let x = json_converter::convert_to_json(body);
    //println!("data:{}", x);
    println!("{}\n{}",
        x.get("location").and_then(|name| name.get("name")).unwrap(), 
        x.get("current").and_then(|temp| temp.get("temp_c")).unwrap());
    println!("Warsaw");
    */
    Ok(())
}
