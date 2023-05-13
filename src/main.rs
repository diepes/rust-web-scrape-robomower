use tokio;
use reqwest;
use serde::Deserialize;
use log::debug;
use log::error;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Start!");
    for i in 1..=99 {
        debug!("i = {i}");
        let url = format!(
            "{url_base}/{uri}?{query}",
            // go check out her latest album. It's 🔥
            url_base = "https://www.yardforce-tools.com",
            uri = "WebData/GetMPageImgs",
            query = format!("countryId={i}"),
        );
        let count = get(&url).await?;
        info!("Check count={count} url={url}");
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

async fn get(url: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = client
        .get(url)
        //.header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        //.header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::REFERER, "https://www.yardforce-tools.com/");
    debug!("Debug request={:?}",request);
    let response = request.send().await?;
    let text = response.text().await?;
    debug!("response = {:?}  len={}", text, text.len());
    let j: Vec<Products> = serde_json::from_str(&text)?;
    debug!("{:?} {}", j, j.len());
    Ok(j.len())
}
