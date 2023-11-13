extern crate clap;

use clap::Parser;
use std::{
    error::Error,
    path::{Path, PathBuf},
    string::String,
};

type CliResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional path to operate on. If not provided, uses current working directory
    path: Option<String>,

    /// Track dev dependencies (default: false)
    #[arg(short, long, default_value = "false")]
    dev: bool,

    /// Times the execution of the command (default: false)
    #[arg(short, long, default_value = "false")]
    timer: bool,
}

/// Config holds the configuration for the application
#[derive(Debug)]
pub struct Config {
    src_path: PathBuf,
    toml_path: PathBuf,
    dev: bool,
    timer: bool,
}

/// get_args parses the command line arguments and returns a Config struct
pub fn get_args() -> CliResult<Config> {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or_else(|| ".".to_string());
    let dev = cli.dev;
    let timer = cli.timer;

    // TODO add path validation
    let src_path = Path::new(&path).to_path_buf();
    let toml_path = Path::new(&path).join("pyproject.toml").to_path_buf();

    Ok(Config {
        src_path,
        toml_path,
        dev,
        timer,
    })
}

/// run executes the application
pub fn run(config: Config) -> CliResult<()> {
    println!("hello world");
    println!("config.src_path: {:?}", config.src_path.display());
    println!("config.toml_path: {:?}", config.toml_path.display());
    println!("config.dev: {:?}", config.dev);
    println!("config.timer: {:?}", config.timer);
    Ok(())
}
