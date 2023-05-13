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
    get(&url).await.expect("Web error");
    Ok(())
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
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    //let body = client.get(url).await?.text().await?;

    println!("response = {:?}  len={}", response,0);
    Ok(())
}
