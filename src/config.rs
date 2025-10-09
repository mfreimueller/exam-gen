use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub(crate) base_file: String,
    pub(crate) questions: String,
}

impl Config {}

pub fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}