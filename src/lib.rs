mod poetry;
mod python_ast;
mod python_std_lib;
pub mod validators;

extern crate clap;

use clap::Parser;
use poetry::get_dependencies_from_pyproject;
use python_ast::get_imports_from_src;
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    string::String,
    time::Instant,
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

    /// Verbose mode (default: false)
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

/// Config holds the configuration for the application
#[derive(Debug)]
pub struct Config {
    src_path: PathBuf,
    toml_path: PathBuf,
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
        verbose,
    })
}

/// run executes the application
pub fn run(config: Config) -> CliResult<()> {
    let start = Instant::now();

    let manifest_deps = get_dependencies_from_pyproject(config.toml_path, config.dev);
    if config.verbose {
        let log_statement = "Manifest dependencies: ";
        print_keys(&manifest_deps, log_statement);
    }
    let import_stmts = get_imports_from_src(&config.src_path);
    if config.verbose {
        let log_statement = "Import statements: ";
        print_keys(&import_stmts, log_statement);
    }

    let unused_deps = find_unused_manifest_deps(manifest_deps, import_stmts);
    if unused_deps.len() == 0 {
        println!("======================================");
        println!("No unused dependencies found.");
    } else {
        println!("======================================");
        println!("Possible unused manifest dependencies: ");
        for dep in unused_deps.iter() {
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

fn print_keys<V>(manifest_deps: &HashMap<String, V>, log_statement: &str) {
    println!("======================================");
    println!("{}", log_statement);
    for (key, _value) in manifest_deps.iter() {
        println!("{}", key);
    }
}

fn find_unused_manifest_deps(
    manifest_deps: HashMap<String, serde_json::Value>,
    import_stmts: HashMap<String, bool>,
) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for (key, _value) in manifest_deps.iter() {
        if !import_stmts.contains_key(key) {
            result.push(key.to_string());
        }
    }
    result
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn test_find_unused_manifest_deps() {
        let mut manifest_deps = HashMap::new();
        manifest_deps.insert(
            "unused_dep".to_string(),
            serde_json::Value::String("1.0.0".to_string()),
        );
        manifest_deps.insert(
            "used_dep".to_string(),
            serde_json::Value::String("1.0.0".to_string()),
        );

        let mut import_stmts = HashMap::new();
        import_stmts.insert("used_dep".to_string(), true);

        let unused_deps = find_unused_manifest_deps(manifest_deps, import_stmts);

        assert_eq!(unused_deps, vec!["unused_dep"]);
    }
}
