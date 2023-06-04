// PESmit 2023-05 retrieve web json from OpenMower manufactur website

use crate::query_url;

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
    log::info!("START countryId={}&secondCId={}", country_id, second_cid);
    let query = format!("?countryId={}&secondCId={}", country_id, second_cid);
    let url = format!(
        "{url_base}/{uri}{query}",
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetProducts",
        query = query,
    );
    let product_classes: Vec<ProductThirdClass> =
        serde_json::from_str(query_url::get(&url).await?.as_str())?;

    log::debug!(
        "Found {} Product_classes_3rd {}",
        product_classes.len(),
        product_classes
            .iter()
            .map(|prod| { format!("{}-{}", prod.third_class_name, prod.products.len()) })
            .collect::<Vec<String>>()
            .join(", ")
    );
    log::info!("done countryId={}&secondCId={}", country_id, second_cid);
    Ok(product_classes)
}
