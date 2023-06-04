// PESmit 2023-05 retrieve web json from OpenMower manufactur website
mod q_g_c;
pub mod query_url;
mod write_to_file;
use chrono::{DateTime, Local};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let t_start = Instant::now();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Start main.");
    let mut f = write_to_file::OutFile::new("./api-dump.yaml").await;
    f.write("api_data:".to_string()).await;

    let area_records = q_g_c::q_g_c().await?;

    println!("");
    f.write(serde_yaml::to_string(&area_records)?).await;
    println!(
        "# Output api data to {} took {:?} {}",
        f.file_name,
        t_start.elapsed(),
        chrono::Local::now()
    );

    Ok(())
}
