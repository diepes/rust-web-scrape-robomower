// PESmit 2023-05 retrieve web json from OpenMower manufactur website
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    query_counties(1, 25).await?;
    Ok(())
}
async fn query_counties(start: usize, end: usize) -> anyhow::Result<()> {
    log::info!("query_counties {start}..{end} Info:202305 countryId's 1, 6, 7, 8, 9, 10, 14, 17, 18, 19, 24");
    let mut handles = Vec::new();
    for i in start..=end {
        log::debug!("i = {i}");
        let query = format!("countryId={i}");
        let url = format!(
            "{url_base}/{uri}?{query}",
            // go check out her latest album. It's 🔥
            url_base = "https://www.yardforce-tools.com",
            uri = "WebData/GetMPageImgs",
            query = query,
        );
        handles.push((i, url.clone(), tokio::spawn(get(url))));
    }
    for (i, url, handle) in handles {
        let count = handle.await.unwrap()?;
        if count > 0 {
            log::info!("🔥 i={i} count={count} url={url}");
        } else {
            log::debug!("* i={i} count={count} url={url}");
        };
    }
    Ok(())
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct Products {
    //{"ProductId":149,"StyleOneUrl":"6d7...22c.jpg","StyleTwoUrl":"d11...bf9.jpg"},
    product_id: u32,
    style_one_url: String,
    style_two_url: String,
}

async fn get(url: String) -> anyhow::Result<usize> {
    let client = reqwest::Client::new();
    let request = client
        .get(url)
        //.header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        //.header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::REFERER, "https://www.yardforce-tools.com/");
    log::debug!("Debug request={:?}", request);
    let response = request.send().await?;
    let text = response.text().await?;
    log::debug!("response = {:?}  len={}", text, text.len());
    let j: Vec<Products> = serde_json::from_str(&text)?;
    log::debug!("{:?} {}", j, j.len());
    Ok(j.len())
}
