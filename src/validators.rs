use std::{
    error, fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum PathError {
    InvalidPath,
    MissingPyprojectToml,
    NonSupportedTooling,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PathError::InvalidPath => f.write_str("Invalid path provided"),
            PathError::MissingPyprojectToml => f.write_str("Unable to find a pyproject.toml file"),
            PathError::NonSupportedTooling => f.write_str("This does not appear to be a Poetry project (no poetry.lock file). Only Poetry is supported at this time. Isn't Python packaging fun? :)"),
        }
    }
}

impl error::Error for PathError {}

/// Validates the path provided by the user:
/// - Checks the path exists
/// - Checks the path contains a pyproject.toml file
/// - Checks the path is a Poetry project (has a poetry.lock file)
pub fn valid_python_path(source_code_path: &String) -> Result<PathBuf, PathError> {
    let valid_path = Path::new(&source_code_path);
    let ok_path = provided_path(valid_path)?;
    if !ok_path {
        return Err(PathError::InvalidPath);
    }

    let ok_toml_file = check_pyproject_file_exists(valid_path)?;
    if !ok_toml_file {
        return Err(PathError::MissingPyprojectToml);
    }

    let ok_poetry_project = check_is_poetry_project(valid_path)?;
    if !ok_poetry_project {
        return Err(PathError::NonSupportedTooling);
    }

    let path_buf = valid_path.to_path_buf();
    Ok(path_buf)
}

fn provided_path(source_code_path: &Path) -> Result<bool, PathError> {
    match fs::metadata(source_code_path) {
        Ok(_) => Ok(true),
        Err(_) => Err(PathError::InvalidPath),
    }
}

fn check_pyproject_file_exists(source_code_path: &Path) -> Result<bool, PathError> {
    let toml_file_path: PathBuf = Path::new(&source_code_path).join("pyproject.toml");
    match fs::metadata(&toml_file_path) {
        Ok(_) => Ok(true),
        Err(_) => Err(PathError::MissingPyprojectToml),
    }
}

fn check_is_poetry_project(source_code_path: &Path) -> Result<bool, PathError> {
    let lock_file_path: PathBuf = Path::new(&source_code_path).join("poetry.lock");
    match fs::metadata(&lock_file_path) {
        Ok(_) => Ok(true),
        Err(_) => Err(PathError::NonSupportedTooling),
    }
}
