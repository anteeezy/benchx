use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    pub command: String,
    #[arg(long)]
    pub iterations: usize,
    #[arg(long, default_value_t = 1)]
    pub concurrency: usize,
    #[arg(long, default_value_t = 1000)]
    pub timeout: usize,
}

pub fn parse_args() -> Config {
    Config::parse()
}