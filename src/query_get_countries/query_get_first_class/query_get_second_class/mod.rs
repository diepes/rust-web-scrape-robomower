// PESmit 2023-05 retrieve web json from OpenMower manufactur website

//use crate::query_get_third_class::ProductThirdClass;
use crate::query_url;

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
    let mut product_classes: Vec<Product2ndClass> =
        serde_json::from_str(query_url::get(&url).await?.as_str())?;
    log::debug!(
        "Found {} 2ndProductClasses {}",
        product_classes.len(),
        product_classes
            .iter()
            .map(|prod| { format!("{}-{}", prod.id, prod.class_name) })
            .collect::<Vec<String>>()
            .join(", ")
    );
    let mut my_fut3: Vec<(
        &mut Vec<query_get_third_class::ProductThirdClass>,
        tokio::task::JoinHandle<
            Result<Vec<query_get_third_class::ProductThirdClass>, anyhow::Error>,
        >,
    )> = vec![];
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
            my_fut3.push((
                &mut second.third_class,
                tokio::spawn(query_get_third_class::query_get_products(
                    country_id, second.id,
                )),
            ));
        }
    }
    for (third_class, fut) in my_fut3 {
        third_class.extend(fut.await??);
    }
    Ok(product_classes)
}
