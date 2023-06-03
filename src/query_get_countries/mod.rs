// PESmit 2023-05 retrieve web json from OpenMower manufactur website

pub mod query_get_first_class;
use crate::query_url;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct CountryRecord {
    //{"AreaName":"International","countries":[{"Id":14,"CountryName":"English","Link":""}]},
    pub id: usize,
    pub country_name: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub link: String,
    #[serde(skip_deserializing)] //add data later
    pub first_class: Vec<query_get_first_class::ProductClass>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AreaRecord {
    pub area_name: String,
    #[serde(rename = "Countrys")]
    pub countries: Vec<CountryRecord>,
}

//# curl 'https://www.yardforce-tools.com/WebData/GetCountry' \
//#   -H 'authority: www.yardforce-tools.com' \
//#   -H 'accept: application/json' \
//#   -H 'referer: https://www.yardforce-tools.com/Mobile_Web/Europe/Deutschland/Products.html' \
//#   --compressed

pub async fn query_get_countries() -> anyhow::Result<Vec<AreaRecord>> {
    log::info!("query_get_countries");
    let url = format!(
        "{url_base}/{uri}",
        url_base = "https://www.yardforce-tools.com",
        uri = "WebData/GetCountry",
    );

    let mut area_records: Vec<AreaRecord> =
        serde_json::from_str(query_url::get(&url).await?.as_str())?;

    log::info!("Found {} area's START", area_records.len(),);
    let mut my_futures: Vec<(
        &mut Vec<query_get_first_class::ProductClass>,
        tokio::task::JoinHandle<Result<Vec<query_get_first_class::ProductClass>, anyhow::Error>>,
    )> = vec![];
    for area in &mut area_records {
        for country in &mut area.countries {
            let country_id = country.id;
            // save mut class and future query
            my_futures.push((
                &mut country.first_class,
                tokio::spawn(query_get_first_class::query_get_first_classes(country_id)),
            ));
        }
    }
    // retrieve mut class and add values from future query
    for (first_class, fut) in my_futures {
        first_class.extend(fut.await??); //1st? await, 2nd? result
    }
    log::info!("Found {} area's done", area_records.len(),);
    Ok(area_records)
}
