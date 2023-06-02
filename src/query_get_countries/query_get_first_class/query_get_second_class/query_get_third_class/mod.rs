// PESmit 2023-05 retrieve web json from OpenMower manufactur website

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ProductThirdClass {
    //[{"ThridClassName":"Robotic Mower","Products":[{"ProductId":598,
    #[serde(rename = "ThridClassName")]
    pub third_class_name: String,
    pub products: Vec<Products>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Products {
    //[{"ProductId":598,"ProductName":"SA500ECO","IconImgUrl":"e29dffa4-95d9-4111-ad82-5a7c37a497d4.jpg","ThirdClassId":223,"ThirdAttrs":[{"AttrId":0,"AttrName":"Battery","AttrValue":"Lithium-Ion Battery, 28 V / 2.0 Ah","IsSpecial":false},{"AttrId":0,"AttrName":"Power Supply","AttrValue":"Input 110-240 V AC, 50 / 60 Hz, Output 32 V / 1.5A d.c. (IP67)","IsSpecial":false},{"AttrId":0,"AttrName":"Cutting Width","AttrValue":"180 mm","IsSpecial":false},{"AttrId":0,"AttrName":"Cutting Height, min-max","AttrValue":"20 - 60 mm","IsSpecial":false}]},
    pub product_id: usize,
    pub product_name: String,
    pub icon_img_url: String,
    pub third_class_id: usize,
    pub third_attrs: Vec<Attributes>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Attributes {
    //[{"AttrId":0,"AttrName":"Battery","AttrValue":"Lithium-Ion Battery, 28 V / 2.0 Ah","IsSpecial":false},
    pub attr_id: usize,
    pub attr_name: String,
    pub attr_value: String,
    pub is_special: bool,
}
pub async fn query_get_products(
    country_id: usize,
    second_cid: usize,
) -> anyhow::Result<Vec<ProductThirdClass>> {
    log::info!("query_get_products_third_class");
    let query = format!("?countryId={}&secondCId={}", country_id, second_cid);
    let url = format!(
        "{url_base}/{uri}{query}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetProducts",
        query = query,
    );
    let product_classes = get(url).await?;
    log::info!(
        "Found {} Product_classes_3rd {}",
        product_classes.len(),
        product_classes
            .iter()
            .map(|prod| { format!("{}-{}", prod.third_class_name, prod.products.len()) })
            .collect::<Vec<String>>()
            .join(", ")
    );
    Ok(product_classes)
}

async fn get(url: String) -> anyhow::Result<Vec<ProductThirdClass>> {
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
    let data: Vec<ProductThirdClass> =
        serde_json::from_str(&text).expect("Failed to parse json response.");

    log::debug!("data = {:#?} len={}", data, data.len());
    Ok(data)
}
