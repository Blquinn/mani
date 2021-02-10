use std::collections::HashSet;

use lazy_static::lazy_static;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub origins: HashSet<String>,
}

fn load_settings() -> Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).expect("Failed to load config.")
        .merge(config::Environment::with_prefix("MANI")).expect("Failed to load config env vars.");

    settings.try_into::<Config>()
        .expect("Failed to convert settings into map.")
}

lazy_static! {
    pub static ref SETTINGS: Config = load_settings();
}
