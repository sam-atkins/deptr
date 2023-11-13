mod poetry;
mod python_ast;
mod validators;

extern crate clap;

use clap::Parser;
use poetry::get_dependencies_from_pyproject;
use python_ast::get_imports_from_src;
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

    let path_result = validators::valid_python_path(&path);
    let src_path: PathBuf;
    let toml_path: PathBuf;
    match path_result {
        Ok(valid_path) => {
            src_path = valid_path;
            toml_path = Path::new(&src_path).join("pyproject.toml");
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    Ok(Config {
        src_path,
        toml_path,
        dev,
        timer,
    })
}

/// run executes the application
pub fn run(config: Config) -> CliResult<()> {
    if config.dev {
        println!("Tracking dev dependencies not currently supported. Proceeding with Production dependencies only.");
        println!("");
    }

    let manifest_deps = get_dependencies_from_pyproject(config.toml_path);
    println!("{:?}", manifest_deps);
    let import_stmts = get_imports_from_src(&config.src_path);
    println!("{:?}", import_stmts);

    Ok(())
}
