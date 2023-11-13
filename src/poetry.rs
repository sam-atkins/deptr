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
}
// TODO add support for dev-dependencies: group.dev.dependencies & older formats
// # new poetry
// [tool.poetry.group.dev.dependencies]

// # older poetry
// [tool.poetry.dev-dependencies]

/// Reads the pyproject.toml file and returns the dependencies
pub fn get_dependencies_from_pyproject(
    toml_file_path: PathBuf,
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

    // NOTES:
    // - A few dependencies are named `python-something` but imported as `something` so we strip `python-` from the name
    // - A few dependencies have dashes but are imported using underscores so we replace `-` with `_`
    pyproject_dependencies = pyproject_dependencies
        .into_iter()
        .map(|(k, v)| {
            if k.starts_with("python-") {
                (k[7..].to_string(), v)
            } else if k.contains("-") {
                (k.replace("-", "_"), v)
            } else {
                (k, v)
            }
        })
        .collect();

    return pyproject_dependencies;
}

#[test]
fn test_get_dependencies_from_pyproject() {
    let toml_file_path: PathBuf = PathBuf::from("tests/fixtures/pyproject.toml");
    let dependencies = get_dependencies_from_pyproject(toml_file_path);
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
