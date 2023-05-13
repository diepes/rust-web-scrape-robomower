// PESmit 2023-05 retrieve web json from OpenMower manufactur website
// mod query_webdata;
mod query_get_country;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Start main.");
    //query_webdata::query_counties(1, 25).await?;

    query_get_country::query_get_countrys().await?;
    Ok(())
}
