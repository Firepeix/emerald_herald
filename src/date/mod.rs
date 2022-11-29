use chrono::DateTime as ChronoDateTime;
use serde::{ Serialize};

pub(crate) struct DateTime(ChronoDateTime<chrono::Local>);

impl DateTime {
    pub fn now() -> Self {
        DateTime(chrono::Local::now())
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(format!("{}", self.0).split('.').next().unwrap_or("NOT A DATE"))
    }
}