// PESmit 2023-05 retrieve web json from OpenMower manufactur website
mod q_g_c;
pub mod query_url;
mod write_to_file;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Start main.");
    let mut f = write_to_file::OutFile::new("./api-dump.yaml").await;
    f.write("api_data:".to_string()).await;

    let area_records = q_g_c::q_g_c().await?;
    f.write(serde_yaml::to_string(&area_records)?).await;

    Ok(())
}

async fn print_pretty_countries(area_records: &[q_g_c::AreaRecord]) {
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
