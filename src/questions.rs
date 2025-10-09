use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use crate::config::Config;

pub fn load_questions(config: &Config) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let mut dir_file_map: HashMap<String, Vec<String>> = HashMap::new();

    let current_dir = env::current_dir()?;
    let base_path = current_dir.join(PathBuf::from(&config.questions));
    let difficulty_levels = get_difficulty_levels(current_dir, &config.questions)?;

    for level in difficulty_levels {
        let level_path = base_path.join(&level);
        let mut files: Vec<String> = Vec::new();

        for entry_result in fs::read_dir(&level_path)? {
            let entry = entry_result?;
            files.push(entry.path().display().to_string());
        }

        dir_file_map.insert(level, files);
    }

    Ok(dir_file_map)
}

fn get_difficulty_levels(current_dir: PathBuf, questions_location: &String) -> anyhow::Result<Vec<String>> {
    let mut difficulty_levels: Vec<String> = Vec::new();

    let base_path = current_dir.join(PathBuf::from(questions_location));
    for entry_result in fs::read_dir(base_path)? {
        let entry = entry_result?;
        if entry.file_type()?.is_dir() {
            difficulty_levels.push(entry
                .file_name()
                .into_string()
                .unwrap());
        }
    }

    Ok(difficulty_levels)
}