// PESmit 2023-05 retrieve web json from OpenMower manufactur website

struct QueryRecord {
    num: usize,
    count: usize,
    url: String,
}

pub async fn query_counties(start: usize, end: usize) -> anyhow::Result<()> {
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
    let mut url_with_values = Vec::new();
    for (i, url, handle) in handles {
        let (count, data) = handle.await.unwrap()?;
        if count > 0 {
            log::info!("🔥 i={i} count={count} url={url}");
            url_with_values.push(QueryRecord {num: i, count: data.len(), url: url } );
        } else {
            log::debug!("* i={i} count={count} url={url}");
        };
    }
    log::info!(
        "Found {} url's with data id's={}",
        url_with_values.len(),
        url_with_values
            .into_iter()
            .map(|r: QueryRecord| format!("{}({})", r.num, r.count) )
            .collect::<Vec<String>>()
            .join(",")
    );
    Ok(())
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct Product {
    //{"ProductId":149,"StyleOneUrl":"6d7...22c.jpg","StyleTwoUrl":"d11...bf9.jpg"},
    product_id: u32,
    style_one_url: String,
    style_two_url: String,
}

async fn get(url: String) -> anyhow::Result<(usize, Vec<Product>)> {
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
    let data: Vec<Product> = serde_json::from_str(&text)?;
    log::debug!("{:?} {}", data, data.len());
    Ok((data.len(), data))
}
