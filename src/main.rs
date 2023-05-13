//use tokio::main;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{url_base}/{uri}?{query}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri ="WebData/GetMPageImgs",
        query = "countryId=18"
    );
    println!("Check url={url}");
    get(&url).await?;
    Ok(())
}

use serde::{Deserialize,};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct Products {
    //{"ProductId":149,"StyleOneUrl":"6d7...22c.jpg","StyleTwoUrl":"d11...bf9.jpg"},
    product_id: u32,
    style_one_url: String,
    style_two_url: String,
}

async fn get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = client
        .get(url)
        //.header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        //.header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::REFERER, "https://www.yardforce-tools.com/");
    //println!("Debug request={:?}",request);

        // confirm the request using send()
    let response = request
        .send()
        .await?;
        // the rest is the same!
        //.unwrap();
    let text = response
        .text()
        .await?;
        //.unwrap();
    //let body = client.get(url).await?.text().await?;
    //println!("response = {:?}  len={}", text, text.len());
    let j: Vec<Products> = serde_json::from_str(&text)?;
    println!("{:?}",j);
    Ok(())
}
