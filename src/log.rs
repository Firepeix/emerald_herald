use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use color_eyre::Result;

pub fn install() -> Result<()>{
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .flatten_event(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}


