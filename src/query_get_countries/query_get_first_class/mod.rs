// PESmit 2023-05 retrieve web json from OpenMower manufactur website

pub mod query_get_second_class;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ProductClass {
    //# [{"Id":82,"ClassName":"Robotermäher","IconImg1":"b4db2dad-5d47-4f9b-8288-44910eb354a6.jpg","IconImg2":"4801f22c-3e62-4502-8963-b4b07a693425.jpg"},
    pub id: usize,
    pub class_name: String,
    #[serde(skip_deserializing)]
    pub second_class: Vec<query_get_second_class::Product2ndClass>,
}

pub async fn query_get_first_classes(country_id: usize) -> anyhow::Result<Vec<ProductClass>> {
    log::info!("query_get_first_classes");
    let query = format!("?countryId={}", country_id);
    let url = format!(
        "{url_base}/{uri}{query}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetFirstClasses",
        query = query,
    );
    let mut product_classes = get(url).await?;
    for product_class in &mut product_classes {
        let second_vec =
            query_get_second_class::query_get_second_classes(country_id, product_class.id).await?;
        product_class.second_class.extend(second_vec);
    }
    log::info!(
        "Found {} ProductClasses {}",
        product_classes.len(),
        product_classes
            .iter()
            .map(|prod| { format!("{}-{}", prod.id, prod.class_name) })
            .collect::<Vec<String>>()
            .join(", ")
    );
    Ok(product_classes)
}

async fn get(url: String) -> anyhow::Result<Vec<ProductClass>> {
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
    let data: Vec<ProductClass> =
        serde_json::from_str(&text).expect("Failed to parse json response.");

    log::debug!("data = {:#?} len={}", data, data.len());
    Ok(data)
}
