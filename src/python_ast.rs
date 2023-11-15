use rustpython_parser::{ast, Parse};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::string::String;

use super::python_std_lib::is_std_lib_module;

const EXCLUDED_DIRS: [&str; 4] = ["venv", ".pytest_cache", ".ruff_cache", ".venv"];

/// Recursively walks the path provided, parses all .py files and
/// returns a hashmap of all non-std lib imports
pub fn get_imports_from_src(directory_path: &Path) -> HashMap<String, bool> {
    let ext = "py";

    return find_files_with_extension(directory_path, ext);
}

fn find_files_with_extension(dir: &Path, extension: &str) -> HashMap<String, bool> {
    let mut result: HashMap<String, bool> = HashMap::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                // Check if the directory should be excluded
                if path.is_dir() && EXCLUDED_DIRS.contains(&file_name.as_str()) {
                    continue;
                }

                if path.is_file() {
                    if let Some(file_extension) = path.extension() {
                        if file_extension == extension {
                            let new_imports = get_imports_from_python_module(&path);
                            result.extend(new_imports);
                        }
                    }
                } else if path.is_dir() {
                    result.extend(find_files_with_extension(&path, extension));
                }
            }
        }
    }

    result
}

/// Reads the Python modules and the import statements, filters out standard lib modules and
/// returns the imports
fn get_imports_from_python_module(module_path: &PathBuf) -> HashMap<String, bool> {
    let mut imports: HashMap<String, bool> = HashMap::new();
    let module_str = module_path.to_str().unwrap();
    let python_source = fs::read_to_string(module_path).expect("Failed to read Python file");

    let python_statements = ast::Suite::parse(&python_source, module_str).unwrap();

    for statement in python_statements.iter() {
        match statement {
            ast::Stmt::Import(import_stmt) => {
                import_stmt.names.iter().for_each(|name| {
                    if !is_std_lib_module(name.name.as_str()) {
                        imports.insert(name.name.as_str().to_string(), true);
                    }
                });
            }
            ast::Stmt::ImportFrom(import_from_stmt) => {
                import_from_stmt.module.as_ref().map(|module| {
                    if !is_std_lib_module(&module.as_str()) {
                        let module_name = &module.as_str().split('.').next().unwrap().to_string();
                        imports.insert(module_name.to_string(), true);
                    }
                });
            }
            // no other use cases
            _ => {}
        }
    }
    return imports;
}
