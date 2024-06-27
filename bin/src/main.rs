use anyhow::Context;
use anyhow::Ok;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("failed to load .env file")?;

    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]",
    )?;
    let time_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(time_offset, timer);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(timer)
        .init();

    tracing::info!("{}", "Hello, world!");

    Ok(())
}
