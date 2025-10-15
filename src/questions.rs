use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use crate::config::Config;

pub fn load_questions(config: &Config) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let mut dir_file_map: HashMap<String, Vec<String>> = HashMap::new();

    let current_dir = env::current_dir()?;
    let base_path = current_dir.join(PathBuf::from(&config.core.questions_dir));

    // TODO: find way to skip .clone()
    for level in config.questions.categories.clone() {
        let level_path = base_path.join(&level);
        let mut files: Vec<String> = Vec::new();

        for entry_result in fs::read_dir(&level_path)? {
            let entry = entry_result?;
            let path = entry.path().display().to_string();
            let question = format!("{path}");

            files.push(question);
        }

        dir_file_map.insert(level, files);
    }

    Ok(dir_file_map)
}
