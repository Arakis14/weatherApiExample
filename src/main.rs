mod key;
/*
pub fn key() -> &'static str {
  return "API_KEY";
}
*/
mod json_converter;
use reqwest::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = key::key();
    let request = format!(
        "http://api.weatherapi.com/v1/current.json?key={}{}",
        api_key, "&q=Warsaw"
    );
    let response = reqwest::get(&request).await?;
    let body = response.text().await.unwrap();
    let x = json_converter::convert_to_json(body);
    //println!("data:{}", x);
    println!("{}\n{}",
        x.get("location").and_then(|name| name.get("name")).unwrap(), 
        x.get("current").and_then(|temp| temp.get("temp_c")).unwrap());
    println!("Warsaw");
    Ok(())
}
