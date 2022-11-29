use tracing::Level;
use tracing_subscriber::{Registry};
use color_eyre::Result;
use tracing_subscriber::layer::SubscriberExt;
use crate::log::{format::FormattingLayer, storage::JsonStorageLayer};
use tracing_subscriber::fmt::writer::MakeWriterExt;

mod storage;
mod format;


pub fn install() -> Result<()>{
    let writer = std::io::stdout.with_max_level(Level::INFO);
    let formatting_layer = FormattingLayer::new("emerald_herald".into(), writer);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);

    //let subscriber = FmtSubscriber::builder()
    //    .with_max_level(Level::INFO)
    //    .json()
    //    .flatten_event(true)
    //    .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}


