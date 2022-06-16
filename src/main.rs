mod key;
/*
pub fn key() -> &'static str {
  return "API_KEY";
}
*/
use reqwest::Result;
//use serde::Deserialize;
/*
#[derive(Deserialize, Debug)]
struct WeatherData {
    location: {
        name: String,
        region: String,
        country: String,
        lat: f64,
        lon: f64,
        tz_id: String,
        location_epoch: i64,
        location_time: String,
    }
}
*/

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = key::key();
    let request = format!("http://api.weatherapi.com/v1/current.json?key={}{}", api_key,"&q=London");
    let response = reqwest::get(request)
        .await?
        .text()
        .await?;
    println!{"{:?}", response};
    //let parsed_response: Vec<Weather> = response.json().await?;
    //println!("{}", parsed_response);
    Ok(())
}
