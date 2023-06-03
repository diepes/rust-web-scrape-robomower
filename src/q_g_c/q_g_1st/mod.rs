// PESmit 2023-05 retrieve web json from OpenMower manufactur website

pub mod q_g_2nd;
use crate::query_url;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ProductClass {
    //# [{"Id":82,"ClassName":"Robotermäher","IconImg1":"b4db2dad-5d47-4f9b-8288-44910eb354a6.jpg","IconImg2":"4801f22c-3e62-4502-8963-b4b07a693425.jpg"},
    pub id: usize,
    pub class_name: String,
    #[serde(skip_deserializing)]
    pub second_class: Vec<q_g_2nd::Product2ndClass>,
}

pub async fn q_g_1st(area_name: String, country_id: usize) -> anyhow::Result<Vec<ProductClass>> {
    log::info!("START {}-{}", area_name, country_id);
    let query = format!("?countryId={}", country_id);
    let url = format!(
        "{url_base}/{uri}{query}",
        // go check out her latest album. It's 🔥
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetFirstClasses",
        query = query,
    );
    let mut my_fut2: Vec<(
        &mut Vec<q_g_2nd::Product2ndClass>,
        tokio::task::JoinHandle<
            Result<Vec<q_g_2nd::Product2ndClass>, anyhow::Error>,
        >,
    )> = vec![];

    let mut product_classes: Vec<ProductClass> =
        serde_json::from_str(query_url::get(&url).await?.as_str())?;

    for product_class in &mut product_classes {
        //let second_vec =
        my_fut2.push((
            &mut product_class.second_class,
            tokio::spawn(q_g_2nd::q_g_2ndes(
                country_id,
                product_class.id,
            )),
        ));
    }
    for (second_class, fut) in my_fut2 {
        second_class.extend(fut.await??);
    }
    log::debug!(
        "Found {} ProductClasses {}",
        product_classes.len(),
        product_classes
            .iter()
            .map(|prod| { format!("{}-{}", prod.id, prod.class_name) })
            .collect::<Vec<String>>()
            .join(", ")
    );
    log::info!("done {}-{}", area_name, country_id);
    Ok(product_classes)
}
