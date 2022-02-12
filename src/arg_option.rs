use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = 3000)]
    pub port: u16,
    #[clap(short, long, default_value = r"./config.yml")]
    pub conf_path: PathBuf,
}
