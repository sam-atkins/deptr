pub mod domain;
mod formatters;
mod poetry;
mod python_ast;
mod python_std_lib;
pub mod validators;

use std::{error::Error, path::PathBuf, string::String, time::Instant};

extern crate clap;
use clap::Parser;

use crate::domain::{PackageManager, PythonProject};

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

    /// Verbose mode (default: false)
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

/// Config holds the configuration for the application
#[derive(Debug)]
pub struct Config {
    src_path: PathBuf,
    dev: bool,
    timer: bool,
    verbose: bool,
}

/// get_args parses the command line arguments and returns a Config struct
pub fn get_args() -> CliResult<Config> {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or_else(|| ".".to_string());
    let dev = cli.dev;
    let timer = cli.timer;
    let verbose = cli.verbose;

    let path_result = validators::valid_python_path(&path);
    let src_path = match path_result {
        Ok(valid_path) => valid_path,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(Config {
        src_path,
        dev,
        timer,
        verbose,
    })
}

/// run executes the application
pub fn run(config: Config) -> CliResult<()> {
    let start = Instant::now();

    // NOTE: currently only supports poetry projects
    let pkg_manager = PackageManager::Poetry;
    let project = PythonProject::new(pkg_manager, config.src_path, config.verbose, config.dev)?;
    let unused_packages = project.get_unused_packages();

    if unused_packages.is_empty() {
        println!("======================================");
        println!("No unused packages found.");
    } else {
        println!("======================================");
        println!("Possible unused manifest packages: ");
        for dep in unused_packages.iter() {
            println!("{}", dep);
        }
    }

    if config.timer {
        let duration = start.elapsed();
        println!("======================================");
        println!("Execution time: {:?}", duration);
    }

    Ok(())
}
