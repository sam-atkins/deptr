//! Poetry is a tool for dependency management and packaging in Python.
//! This module parses the pyproject.toml file and extracts the dependencies.
extern crate toml;

use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
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

/// Reads the pyproject.toml file and returns the dependencies
pub fn get_dependencies_from_pyproject(
    toml_file_path: PathBuf,
    with_dev_deps: bool,
) -> HashMap<String, serde_json::Value> {
    let mut toml_content: String = String::new();
    fs::File::open(toml_file_path)
        .expect("Failed to open file")
        .read_to_string(&mut toml_content)
        .expect("Failed to read file");

    let pyproject: PyProjectToml =
        toml::from_str(&toml_content).expect("Failed to parse pyproject.toml");

    let mut pyproject_dependencies = pyproject.tool.poetry.dependencies.clone();

    // python is included as a dependency in poetry but we don't want to include it
    pyproject_dependencies.remove("python");

    if with_dev_deps {
        let dev_dependencies = get_dev_dependencies(pyproject);
        pyproject_dependencies.extend(dev_dependencies);
    }

    pyproject_dependencies = pyproject_dependencies
        .into_iter()
        .map(|(k, v)| {
            let transformed_k = transform_dep_for_import_matching(&k);
            (transformed_k, v)
            // k = transform_dep_for_import_matching(&k);
        })
        .collect();

    return pyproject_dependencies;
}

fn get_dev_dependencies(pyproject: PyProjectToml) -> HashMap<String, serde_json::Value> {
    let mut all_dev_deps = HashMap::new();
    if let Some(dev_dependencies) = pyproject.tool.poetry.dev_dependencies {
        all_dev_deps.extend(dev_dependencies.into_iter());
    }
    if let Some(group) = pyproject.tool.poetry.group {
        if let Some(dev_dependencies) = group.dev.dependencies {
            all_dev_deps.extend(dev_dependencies.into_iter());
        }
    }

    all_dev_deps
}

#[test]
fn test_get_dependencies_from_pyproject() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(toml_file_path, false);
    assert_eq!(dependencies.len(), 11);
    assert_eq!(
        dependencies.get("fastapi").unwrap(),
        &String::from("^0.104.1")
    );
    assert_eq!(dependencies.get("redis").unwrap(), &String::from("^4.5.5"));
    assert_eq!(
        dependencies.get("sqlalchemy").unwrap(),
        &String::from("^2.0.17")
    );
    assert_eq!(
        dependencies.get("pydantic").unwrap(),
        &String::from("^1.10.9")
    );
    assert_eq!(
        dependencies.get("requests").unwrap(),
        &String::from("^2.31.0")
    );
    assert_eq!(
        dependencies.get("tenacity").unwrap(),
        &String::from("^8.2.2")
    );
    assert_eq!(
        dependencies.get("alembic").unwrap(),
        &String::from("^1.11.1")
    );
    assert_eq!(
        dependencies.get("dotenv").unwrap(),
        &String::from("^0.19.1")
    );
    assert_eq!(
        dependencies.get("scikit_learn").unwrap(),
        &String::from("^1.3.2")
    );
    assert_eq!(dependencies.get("mako").unwrap(), &String::from("^1.3.0"));
}

#[test]
fn test_get_dependencies_from_pyproject_with_dev() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(toml_file_path, true);
    assert_eq!(dependencies.len(), 14);
    assert_eq!(
        dependencies.get("fastapi").unwrap(),
        &String::from("^0.104.1")
    );
    assert_eq!(dependencies.get("pytest").unwrap(), &String::from("^7.4.0"));
}

#[test]
fn test_get_dependencies_from_pyproject_with_dev_from_old_poetry() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/input/old/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(toml_file_path, true);
    assert_eq!(dependencies.len(), 14);
    assert_eq!(
        dependencies.get("fastapi").unwrap(),
        &String::from("^0.104.1")
    );
    assert_eq!(dependencies.get("pytest").unwrap(), &String::from("^7.4.0"));
}

#[test]
fn test_get_dependencies_from_pyproject_with_dev_no_dev_in_poetry() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/input/no_dev/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(toml_file_path, true);
    assert_eq!(dependencies.len(), 11);
    assert_eq!(
        dependencies.get("fastapi").unwrap(),
        &String::from("^0.104.1")
    );
}

/// Transforms the dependency name to improve the likelihood of matching the import statement
/// - A few dependencies are named `python-something` but imported as `something` so we strip
///   `python-` from the name
/// - A few dependencies have dashes but are imported using underscores so we replace `-` with `_`
fn transform_dep_for_import_matching(dep: &str) -> String {
    let mut dep = dep.to_string();
    if dep.starts_with("python-") {
        dep = dep[7..].to_string();
    }
    if dep.contains("-") {
        dep = dep.replace("-", "_");
    }
    dep
}

#[test]
fn test_transform_dep_for_import_matching() {
    let dep = "python-redis";
    let transformed_dep = transform_dep_for_import_matching(dep);
    assert_eq!(transformed_dep, "redis");

    let dep = "python-redis-abc";
    let transformed_dep = transform_dep_for_import_matching(dep);
    assert_eq!(transformed_dep, "redis_abc");

    let dep = "redis-abc";
    let transformed_dep = transform_dep_for_import_matching(dep);
    assert_eq!(transformed_dep, "redis_abc");

    let dep = "redis";
    let transformed_dep = transform_dep_for_import_matching(dep);
    assert_eq!(transformed_dep, "redis");
}
