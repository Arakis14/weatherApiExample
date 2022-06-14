use reqwest::Result;
//use serde::Deserialize;

/*
#[derive(Deserialize, Debug)]
struct Weather {
    body: String,
    content: String,
}
*/

#[tokio::main]
async fn main() -> Result<()> {

    let response = reqwest::get("http://api.weatherapi.com/v1/current.json?key=2e280afadc90462bba4194432221406&q=London")
        .await?
        .text()
        .await?;
    println!{"{:?}", response};
    //let parsed_response: Vec<Weather> = response.json().await?;
    //println!("{}", parsed_response);
    Ok(())
}
