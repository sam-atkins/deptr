//! Poetry is a tool for dependency management and packaging in Python.
//! This module parses Poetry pyproject.toml and lock files to get package dependencies.
extern crate toml;

use serde::Deserialize;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::string::String;

#[derive(Deserialize, Debug)]
struct PyProjectToml {
    tool: Tool,
}

#[derive(Deserialize, Debug)]
struct Tool {
    poetry: Poetry,
}

#[derive(Deserialize, Debug)]
struct Poetry {
    dependencies: HashMap<String, serde_json::Value>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<HashMap<String, serde_json::Value>>,
    group: Option<Group>,
}

#[derive(Deserialize, Debug)]
struct Group {
    dev: Dev,
}

#[derive(Deserialize, Debug)]
struct Dev {
    dependencies: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Deserialize, Debug)]
struct PoetryLock {
    package: Vec<Package>,
}

#[derive(Clone, Deserialize, Debug)]
struct Package {
    name: String,
    extras: Option<HashMap<String, serde_json::Value>>,
}

/// Reads the pyproject.toml file and returns the dependencies
pub fn get_dependencies_from_pyproject(
    toml_file_path: &PathBuf,
    with_dev_deps: bool,
) -> HashSet<String> {
    let mut toml_content: String = String::new();
    fs::File::open(toml_file_path)
        .expect("Failed to open file")
        .read_to_string(&mut toml_content)
        .expect("Failed to read file");

    let pyproject: PyProjectToml =
        toml::from_str(&toml_content).expect("Failed to parse pyproject.toml");

    let mut pyproject_dependencies: HashSet<_> =
        pyproject.tool.poetry.dependencies.keys().cloned().collect();

    // python is included as a dependency in poetry but we don't want to include it
    pyproject_dependencies.remove("python");

    if with_dev_deps {
        let dev_dependencies = get_dev_dependencies(pyproject);
        pyproject_dependencies.extend(dev_dependencies);
    }

    pyproject_dependencies
}

fn get_dev_dependencies(pyproject: PyProjectToml) -> HashSet<String> {
    let mut all_dev_deps = HashSet::new();
    if let Some(dev_dependencies) = pyproject.tool.poetry.dev_dependencies {
        all_dev_deps.extend(dev_dependencies.keys().cloned());
    }
    if let Some(group) = pyproject.tool.poetry.group {
        if let Some(dev_dependencies) = group.dev.dependencies {
            all_dev_deps.extend(dev_dependencies.keys().cloned());
        }
    }

    all_dev_deps
}

/// Checks the lock file for any package extras and returns a HashMap
/// with the package name as the key and a Vec of the extras as the value
/// for example:
/// ```ignore
///  {
///     "pydantic": [
///         "email-validator",
///         "python-dotenv",
///     ],
/// }
/// ```
pub fn check_lock_file_for_package_extras(
    project_path: &PathBuf,
    manifest_packages: &HashSet<String>,
    verbose: bool,
) -> HashMap<String, Vec<String>> {
    let lock_file_path = project_path.join("poetry.lock");
    if !lock_file_path.exists() {
        println!("WARNING: Project has no lock file.");
        return HashMap::new();
    }

    let mut lock_file_content: String = String::new();
    fs::File::open(lock_file_path)
        .expect("Failed to open file")
        .read_to_string(&mut lock_file_content)
        .expect("Failed to read file");

    let lock_file: PoetryLock = toml::from_str(&lock_file_content).unwrap();
    let extras =
        lock_file
            .package
            .iter()
            .fold(HashMap::<String, Vec<String>>::new(), |mut acc, package| {
                if let Some(extras) = &package.extras {
                    for (_key, value) in extras {
                        let array = value.as_array().unwrap();
                        for item in array {
                            let pkg = item.as_str().unwrap();
                            if manifest_packages
                                .iter()
                                .any(|package| pkg.contains(package))
                            {
                                let fmt_pkg = pkg.split('(').next().unwrap_or("").trim();
                                if verbose {
                                    println!(
                                        "Found {} - it is an extra dependency of {}",
                                        fmt_pkg, package.name
                                    );
                                }
                                acc.entry(package.name.clone())
                                    .and_modify(|v| v.push(fmt_pkg.to_string()))
                                    .or_insert_with(|| vec![fmt_pkg.to_string()]);
                            }
                        }
                    }
                }
                acc
            });
    extras
}

#[test]
fn test_get_dependencies_from_pyproject() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/example_project/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(&toml_file_path, false);
    assert_eq!(dependencies.len(), 12);
    assert_eq!(dependencies.get("fastapi"), Some(&"fastapi".to_string()));
    assert_eq!(dependencies.get("redis"), Some(&"redis".to_string()));
    assert_eq!(
        dependencies.get("sqlalchemy"),
        Some(&"sqlalchemy".to_string())
    );
    assert_eq!(dependencies.get("pydantic"), Some(&"pydantic".to_string()));
    assert_eq!(dependencies.get("requests"), Some(&"requests".to_string()));
    assert_eq!(dependencies.get("tenacity"), Some(&"tenacity".to_string()));
    assert_eq!(dependencies.get("alembic"), Some(&"alembic".to_string()));
    assert_eq!(
        dependencies.get("python-dotenv"),
        Some(&"python-dotenv".to_string())
    );
    assert_eq!(
        dependencies.get("scikit-learn"),
        Some(&"scikit-learn".to_string())
    );
    assert_eq!(dependencies.get("mako"), Some(&"mako".to_string()));
}

#[test]
fn test_get_dependencies_from_pyproject_with_dev() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/example_project/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(&toml_file_path, true);
    assert_eq!(dependencies.len(), 15);
    assert_eq!(dependencies.get("fastapi"), Some(&"fastapi".to_string()));
    assert_eq!(dependencies.get("pytest"), Some(&"pytest".to_string()));
}

#[test]
fn test_get_dependencies_from_pyproject_with_dev_from_old_poetry() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/input/old/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(&toml_file_path, true);
    assert_eq!(dependencies.len(), 14);
    assert_eq!(dependencies.get("fastapi"), Some(&"fastapi".to_string()));
    assert_eq!(dependencies.get("pytest"), Some(&"pytest".to_string()));
}

#[test]
fn test_get_dependencies_from_pyproject_with_dev_no_dev_in_poetry() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/input/no_dev/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(&toml_file_path, true);
    assert_eq!(dependencies.len(), 11);
    assert_eq!(dependencies.get("fastapi"), Some(&"fastapi".to_string()));
}

#[test]
fn test_check_lock_file_for_package_extras() {
    let project_path: PathBuf = PathBuf::from("tests/fixtures/input/lockfile");
    let manifest_packages: HashSet<String> = [
        "pydantic".to_string(),
        "sci-kit-learn".to_string(),
        "python-dotenv".to_string(),
        "tenacity".to_string(),
        "fastapi".to_string(),
        "alembic".to_string(),
        "sqlalchemy".to_string(),
        "requests".to_string(),
        "email-validator".to_string(),
        "sentry-sdk".to_string(),
        "mako".to_string(),
        "redis".to_string(),
    ]
    .iter()
    .cloned()
    .collect();

    let mut extras = check_lock_file_for_package_extras(&project_path, &manifest_packages, false);
    for vec in extras.values_mut() {
        vec.sort();
    }

    assert_eq!(extras.len(), 1);
    assert_eq!(
        extras.get("pydantic"),
        Some(&vec![
            "email-validator".to_string(),
            "python-dotenv".to_string(),
        ])
    );
}
