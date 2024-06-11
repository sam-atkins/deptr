use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    string::String,
};

use rustpython_parser::{ast, Parse};

use super::python_std_lib::is_std_lib_module;

const EXCLUDED_DIRS: [&str; 4] = ["venv", ".pytest_cache", ".ruff_cache", ".venv"];

/// Recursively walks the path provided, parses all .py files and
/// returns a HashSet of all Python non-standard library imports
pub fn get_imports_from_src(directory_path: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
    let ext = "py";

    match find_files_with_extension(directory_path, ext) {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

fn find_files_with_extension(
    dir: &Path,
    extension: &str,
) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut result: HashSet<String> = HashSet::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            // let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let file_name = if let Some(file_name) = path.file_name() {
                file_name.to_string_lossy().to_string()
            } else {
                continue;
            };

            // Check if the directory should be excluded
            if path.is_dir() && EXCLUDED_DIRS.contains(&file_name.as_str()) {
                continue;
            }

            if path.is_file() {
                if let Some(file_extension) = path.extension() {
                    if file_extension == extension {
                        let new_imports = get_imports_from_python_module(&path);
                        match new_imports {
                            Ok(imports) => {
                                result.extend(imports);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                match find_files_with_extension(&path, extension) {
                    Ok(new_imports) => {
                        result.extend(new_imports);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Reads the Python modules and the import statements, filters out Python standard
/// library modules and returns the imports
fn get_imports_from_python_module(
    module_path: &PathBuf,
) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut imports: HashSet<String> = HashSet::new();
    let module_str = module_path
        .to_str()
        .ok_or("Failed to convert path to string")?;
    let python_source = fs::read_to_string(module_path)?;

    let python_statements = ast::Suite::parse(&python_source, module_str)?;

    for statement in python_statements.iter() {
        match statement {
            ast::Stmt::Import(import_stmt) => {
                import_stmt.names.iter().for_each(|name| {
                    if !is_std_lib_module(name.name.as_str()) {
                        imports.insert(name.name.as_str().to_string());
                    }
                });
            }
            ast::Stmt::ImportFrom(import_from_stmt) => {
                let results: Result<Vec<_>, _> = import_from_stmt
                    .module
                    .as_ref()
                    .iter()
                    .map(|module| {
                        if !is_std_lib_module(module.as_str()) {
                            let module_name: Result<&str, _> = module
                                .as_str()
                                .split('.')
                                .next()
                                .ok_or("Module name not found");
                            imports.insert(module_name?.to_string());
                        }
                        Ok(()) as Result<(), Box<dyn Error>>
                    })
                    .collect();

                results?;
            }
            // no other use cases
            _ => {}
        }
    }

    Ok(imports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_imports_from_src() {
        let test_path = Path::new("tests/fixtures/example_project");
        let result = get_imports_from_src(test_path).unwrap();
        let expected: HashSet<String> = [
            "requests".to_string(),
            "alembic".to_string(),
            "mako".to_string(),
            "sqlalchemy".to_string(),
            "pydantic".to_string(),
            "fastapi".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(result, expected);
    }
}
