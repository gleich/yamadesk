use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
}

impl Config {
    pub fn read() -> Result<Self> {
        let contents = fs::read_to_string("yamadesk.toml").context("reading from file")?;
        Ok(toml::from_str(&contents).context("parsing toml")?)
    }
}
