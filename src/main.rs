// PESmit 2023-05 retrieve web json from OpenMower manufactur website
// mod query_webdata;
mod query_get_countries;
// mod query_get_first_class;
// mod query_get_second_class;
// mod query_get_third_class;
mod write_to_file;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Start main.");
    let mut f = write_to_file::OutFile::new("./api-dump.yaml").await;
    f.write("test msg:".to_string()).await;

    // let mut yaml = serde_yaml::Serializer::new(String);

    let area_records = query_get_countries::query_get_countries().await?;
    f.write(serde_yaml::to_string(&area_records)?).await;
    // print_pretty_countries(&area_records).await;
    f.write("area_names:".to_string()).await;
    for area in area_records {
        f.write(format!("  {}:", area.area_name)).await;
        for country in area.countries {
            //f.flush().await.expect("Unable to flush to disk");
            f.write(format!("    - name: {}", country.country_name))
                .await;
            f.write(format!("      id: {}", country.id)).await;
            f.write("      classes_1:".to_string()).await;
            let country_id = country.id;
            let first = query_get_countries::query_get_first_class::query_get_first_classes(country_id).await?;
            f.write(serde_yaml::to_string(&first)?).await;
        }
    }

    Ok(())
}

async fn print_pretty_countries(area_records: &[query_get_countries::AreaRecord]) {
    log::info!("Start print_pretty_countries");
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
