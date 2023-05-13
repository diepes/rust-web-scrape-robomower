// PESmit 2023-05 retrieve web json from OpenMower manufactur website

struct QueryRecord {
    num: usize,
    count: usize,
    url: String,
}
#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct CountryRecord {
    //{"AreaName":"International","countries":[{"Id":14,"CountryName":"English","Link":""}]},
    id: usize,
    country_name: String,
    link: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct AreaRecord {
    area_name: String,
    #[serde(rename = "Countrys")]
    countries: Vec<CountryRecord>,
}

//# curl 'https://www.yardforce-tools.com/WebData/GetCountry' \
//#   -H 'authority: www.yardforce-tools.com' \
//#   -H 'accept: application/json' \
//#   -H 'referer: https://www.yardforce-tools.com/Mobile_Web/Europe/Deutschland/Products.html' \
//#   --compressed

pub async fn query_get_countries() -> anyhow::Result<()> {
    log::info!("query_get_countries");
    let url = format!(
        "{url_base}/{uri}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetCountry",
    );
    let area_records = get(url).await.unwrap();
    log::info!("Found {} countries", area_records.len(),);
    print_pretty_countries(area_records).await;
    Ok(())
}
async fn print_pretty_countries(area_records: Vec<AreaRecord>) {
    let mut count_countries = 0;
    for area_record in area_records.iter() {
        print!("Area: {}", area_record.area_name);
        println!(
            "    {}",
            area_record
                .countries
                .iter()
                .map(|country| {
                    count_countries += 1;
                    format!("Country:{}-{}", country.id, country.country_name)
                })
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    println!(" Total Countries {count_countries}");
}

async fn get(url: String) -> anyhow::Result<Vec<AreaRecord>> {
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
    let data: Vec<AreaRecord> =
        serde_json::from_str(&text).expect("Failed to parse json response.");

    log::debug!("data = {:#?} len={}", data, data.len());
    Ok(data)
}
