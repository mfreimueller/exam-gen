mod cli_args;
mod config;
mod questions;

use clap::Parser;
use crate::cli_args::Args;
use anyhow;
use crate::config::{read_config, Config};
use crate::questions::load_questions;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = read_config(args.config_file)?;

    let map = load_questions(&config);

    Ok({})
}
