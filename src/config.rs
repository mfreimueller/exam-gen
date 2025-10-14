use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: CoreConfig,
    pub questions: QuestionsConfig,
}

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub base_file: String,
    pub questions_dir: String,
    pub out_dir: String,
    pub convert_pdf: bool,
}

#[derive(Debug, Deserialize)]
pub struct QuestionsConfig {
    pub categories: Vec<String>,
    pub shuffle: bool,
}

impl Config {}

pub fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}