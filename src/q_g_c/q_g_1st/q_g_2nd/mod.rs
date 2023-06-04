// PESmit 2023-05 retrieve web json from OpenMower manufactur website

//use crate::q_g_3rd::ProductThirdClass;
use crate::query_url;

pub mod q_g_3rd;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Product2ndClass {
    //[{\"Id\":84,\"ClassName\":\"Robotic\",\"IconImg1\":\"91....jpg\",\"IconImg2\":\"c422e2c2-4885-46d9-8a99-1815d543bfc1.jpg\"},
    pub id: usize,
    pub class_name: String,
    pub icon_img1: String,
    pub icon_img2: String,
    #[serde(skip_deserializing)]
    pub third_class: Vec<q_g_3rd::ProductThirdClass>,
    // pub third_class: Option<Vec<q_g_3rd::ProductThirdClass>>,
}

pub async fn q_g_2ndes(
    country_id: usize,
    class_id_1st: usize,
) -> anyhow::Result<Vec<Product2ndClass>> {
    log::info!("START {}-{}", country_id, class_id_1st);
    let query = format!("?countryId={}&firstClassId={}", country_id, class_id_1st);
    let url = format!(
        "{url_base}/{uri}{query}",
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
        &mut Vec<q_g_3rd::ProductThirdClass>,
        tokio::task::JoinHandle<Result<Vec<q_g_3rd::ProductThirdClass>, anyhow::Error>>,
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
                tokio::spawn(q_g_3rd::query_get_products(country_id, second.id)),
            ));
        }
    }
    for (third_class, fut) in my_fut3 {
        third_class.extend(fut.await??);
    }
    log::info!("done {}-{}", country_id, class_id_1st);
    Ok(product_classes)
}
