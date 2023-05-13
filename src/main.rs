//use std::array;

use log::debug;
//use log::error;
use log::info;
use anyhow::{Result};  //generic errors
use reqwest;
use serde::Deserialize;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    query_counties(1, 25).await?;
    Ok(())
}
async fn query_counties(start: usize, end: usize) -> Result<(), Box<dyn std::error::Error>> {
    info!("query_counties {start}..{end} Info:202305 countryId's 1, 6, 7, 8, 9, 10, 14, 17, 18, 19, 24");
    let mut handles = Vec::new();
    for i in start..=end {
        debug!("i = {i}");
        let url = format!(
            "{url_base}/{uri}?{query}",
            // go check out her latest album. It's 🔥
            url_base = "https://www.yardforce-tools.com",
            uri = "WebData/GetMPageImgs",
            query = format!("countryId={i}"),
        );
        handles.push((i, url.clone(), tokio::spawn(get(url))));
    }
    for (i, url, handle) in handles {
        let count = handle.await.unwrap()?;
        if count > 0 {
            info!("🔥 i={i} count={count} url={url}");
        } else {
            debug!("* i={i} count={count} url={url}");
        };
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct Products {
    //{"ProductId":149,"StyleOneUrl":"6d7...22c.jpg","StyleTwoUrl":"d11...bf9.jpg"},
    product_id: u32,
    style_one_url: String,
    style_two_url: String,
}

async fn get(url: String) -> Result<usize> {
    let client = reqwest::Client::new();
    let request = client
        .get(url)
        //.header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        //.header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::REFERER, "https://www.yardforce-tools.com/");
    debug!("Debug request={:?}", request);
    let response = request.send().await?;
    let text = response.text().await?;
    debug!("response = {:?}  len={}", text, text.len());
    let j: Vec<Products> = serde_json::from_str(&text)?;
    debug!("{:?} {}", j, j.len());
    Ok(j.len())
}
