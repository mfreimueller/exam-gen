use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use rand::seq::SliceRandom;
use crate::config::Config;

const SEARCH_PREFIX: &str = "% @insert";

pub struct ExamOptions {
    pub(crate) tex: String,
    pub(crate) difficulty_levels: HashMap<String, i32>
}

pub fn load_exam_options(config: &Config) -> anyhow::Result<ExamOptions> {
    let current_dir = env::current_dir()?;
    let base_path = current_dir.join(PathBuf::from(&config.core.base_file));

    let tex = std::fs::read_to_string(&base_path)?;
    let difficulty_levels = extract_difficulty_levels(&tex);

    Ok(ExamOptions {
        tex,
        difficulty_levels
    })
}

pub fn write_questions_into_tex(tex: &String, questions: &Vec<String>) -> anyhow::Result<String> {
    let mut tex = tex.clone();

    let insert_loc = tex.find(SEARCH_PREFIX).unwrap();

    let mut rnd = rand::rng();
    let mut shuffled_questions: Vec<String> = questions.clone();
    shuffled_questions.shuffle(&mut rnd);

    let mut tex_code = String::new();
    for file_path in shuffled_questions {
        let file_code = fs::read_to_string(file_path)?;
        tex_code = format!("{tex_code}\n\n\n{file_code}");
    }

    tex.insert_str(insert_loc, &tex_code);

    Ok(tex)
}

fn extract_difficulty_levels(tex: &String) -> HashMap<String, i32> {
    let mut difficulty_levels: HashMap<String, i32> = HashMap::new();
    for line in tex.lines() {
        if line.starts_with(SEARCH_PREFIX) {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            if parts.len() >= 4 {
                let num_of_questions = parts[2].parse::<i32>().unwrap();
                let difficulty = parts[3].to_string();
                difficulty_levels.insert(difficulty, num_of_questions);
            }
        }
    }

    difficulty_levels
}