// PESmit 2023-05 retrieve web json from OpenMower manufactur website

struct QueryRecord {
    num: usize,
    count: usize,
    url: String,
}
#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct CountryRecord {
    //{"AreaName":"International","Countrys":[{"Id":14,"CountryName":"English","Link":""}]},
    id: usize,
    country_name: String,
    link: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct AreaRecord {
    area_name: String,
    countrys: Vec<CountryRecord>,
}

//# curl 'https://www.yardforce-tools.com/WebData/GetCountry' \
//#   -H 'authority: www.yardforce-tools.com' \
//#   -H 'accept: application/json' \
//#   -H 'referer: https://www.yardforce-tools.com/Mobile_Web/Europe/Deutschland/Products.html' \
//#   --compressed

pub async fn query_get_countrys() -> anyhow::Result<()> {
    log::info!("query_get_countrys");
    let url = format!(
        "{url_base}/{uri}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetCountry",
    );
    let count = get(url).await.unwrap();
    log::info!("Found {} countries", count,);
    Ok(())
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
    let data: Vec<AreaRecord> = serde_json::from_str(&text).expect("Failed to parse json response.");

    log::debug!("{:?} {}", data, data.len());
    Ok(data.len())
}
