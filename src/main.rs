mod cli_args;
mod config;
mod questions;
mod tex_ops;
mod generator;
mod export_ops;

use clap::Parser;
use crate::cli_args::Args;
use anyhow;
use crate::config::{read_config};
use crate::export_ops::export_tests;
use crate::generator::generate_exams;
use crate::questions::load_questions;
use crate::tex_ops::{load_exam_options};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = read_config(args.config_file)?;

    let questions = load_questions(&config)?;
    let exam_options = load_exam_options(&config)?;

    let tests = generate_exams(args.count, &exam_options, questions)?;

    export_tests(&config, &exam_options, tests)
}
