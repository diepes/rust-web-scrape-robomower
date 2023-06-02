// PESmit 2023-05 retrieve web json from OpenMower manufactur website

//use crate::query_get_third_class::ProductThirdClass;

pub mod query_get_third_class;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Product2ndClass {
    //[{\"Id\":84,\"ClassName\":\"Robotic\",\"IconImg1\":\"91....jpg\",\"IconImg2\":\"c422e2c2-4885-46d9-8a99-1815d543bfc1.jpg\"},
    pub id: usize,
    pub class_name: String,
    pub icon_img1: String,
    pub icon_img2: String,
    #[serde(skip_deserializing)]
    pub third_class: Vec<query_get_third_class::ProductThirdClass>,
    // pub third_class: Option<Vec<query_get_third_class::ProductThirdClass>>,
}

pub async fn query_get_second_classes(
    country_id: usize,
    class_id_1st: usize,
) -> anyhow::Result<Vec<Product2ndClass>> {
    log::info!("query_get_second_classes");
    let query = format!("?countryId={}&firstClassId={}", country_id, class_id_1st);
    let url = format!(
        "{url_base}/{uri}{query}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetSecondClasses",
        query = query,
    );
    let mut product_classes = get(url).await?;
    log::info!(
        "Found {} 2ndProductClasses {}",
        product_classes.len(),
        product_classes
            .iter()
            .map(|prod| { format!("{}-{}", prod.id, prod.class_name) })
            .collect::<Vec<String>>()
            .join(", ")
    );
    for second in &mut product_classes {
        if [
            "Robotic",
            "Robotermäher",
            "Robot-tondeuse",
            "Robots",
            "Robot",
        ]
        .iter()
        .any(|&s| s == second.class_name)
        {
            let products = query_get_third_class::query_get_products(country_id, second.id).await?;
            second.third_class.extend(products);
        }
    }
    Ok(product_classes)
}

async fn get(url: String) -> anyhow::Result<Vec<Product2ndClass>> {
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
    let data: Vec<Product2ndClass> =
        serde_json::from_str(&text).expect("Failed to parse json response.");

    log::debug!("data = {:#?} len={}", data, data.len());
    Ok(data)
}
