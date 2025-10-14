use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "exam-gen.toml")]
    pub(crate) config_file: String,
    #[arg(long)]
    pub(crate) count: i32,
}