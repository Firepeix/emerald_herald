use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use color_eyre::Result;

pub fn install() -> Result<()>{
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
