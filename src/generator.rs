use std::collections::HashMap;
use crate::config::Config;
use crate::tex_ops::ExamOptions;
use itertools::Itertools;
use rand::prelude::*;

pub type Question = String;
pub type Test = Vec<Question>;

pub fn generate_exams(config: &Config, exam_options: &ExamOptions, questions: HashMap<String, Vec<String>>) -> anyhow::Result<Vec<Test>> {
    let pattern = get_pattern(&exam_options);
    let permutations = generate_permutations(pattern, questions);

    let mut tests: Vec<Test> = Vec::new();
    let mut rng = rand::rng();
    for _ in 0..config.student_count {
        let random_idx = rng.random_range(1..permutations.len());
        tests.push(permutations[random_idx].clone());
    }

    Ok(tests)
}

fn generate_permutations(pattern: Vec<String>, questions: HashMap<String, Vec<String>>) -> Vec<Test> {
    let mut position_pools: Vec<Vec<String>> = Vec::new();
    for type_key in &pattern {
        if let Some(question) = questions.get(type_key) {
            position_pools.push(question.clone());
        } else {
            eprintln!("Warning: unknown type {}", type_key);
            return Vec::new()
        }
    }

    let all_permutations = position_pools
        .into_iter()
        .multi_cartesian_product()
        .collect::<Vec<Test>>();

    let unique_permutations: Vec<Test> = all_permutations
        .into_iter()
        .filter(|test| {
            let unique_questions: std::collections::HashSet<_> = test.iter().collect();
            unique_questions.len() == test.len()
        }).collect();

    unique_permutations
}

fn get_pattern(exam_options: &ExamOptions) -> Vec<String> {
    let mut pattern: Vec<String> = Vec::new();
    for difficulty in exam_options.difficulty_levels.clone() {
        for _ in 0..difficulty.1 {
            pattern.push(difficulty.0.clone());
        }
    }

    pattern
}