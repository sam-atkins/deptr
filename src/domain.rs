use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    formatters::reformat_package_name,
    poetry::{check_lock_file_for_package_extras, get_dependencies_from_pyproject},
    python_ast::get_imports_from_src,
};

pub struct PythonProject {
    manifest_packages: HashSet<String>,
    extra_packages: HashMap<String, Vec<String>>,
    import_statements: HashSet<String>,
}

impl PythonProject {
    /// Creates a new PythonProject instance
    pub fn new(project_path: PathBuf, verbose: bool, dev: bool) -> Self {
        let toml_path = project_path.join("pyproject.toml");
        let manifest_packages = get_dependencies_from_pyproject(&toml_path, dev);
        let extra_packages =
            check_lock_file_for_package_extras(&project_path, &manifest_packages, verbose);
        let import_statements = get_imports_from_src(&project_path);

        Self {
            manifest_packages,
            extra_packages,
            import_statements,
        }
    }

    /// Returns a HashSet of unused packages
    pub fn get_unused_packages(&self) -> HashSet<String> {
        let unused_packages = self.find_unused_manifest_packages();
        self.filter_package_extras(unused_packages)
    }

    /// Returns a HashSet of unused packages from the manifest
    fn find_unused_manifest_packages(&self) -> HashSet<String> {
        let manifest_packages_fmt: HashSet<String> = self
            .manifest_packages
            .iter()
            .map(|pkg| reformat_package_name(pkg))
            .collect();

        manifest_packages_fmt
            .difference(&self.import_statements)
            .cloned()
            .collect()
    }

    /// Returns a HashSet of unused packages, filtered by package extras
    fn filter_package_extras(&self, unused_packages: HashSet<String>) -> HashSet<String> {
        let mut result = unused_packages.clone();

        // If a package is installed as an extra of a parent package,
        // and the parent package is used, then the extra package is removed
        // from the unused_packages
        for (pkg, extras) in self.extra_packages.iter() {
            for extra in extras.iter() {
                let extra_pkg = reformat_package_name(&extra);
                if !unused_packages.contains(pkg) && unused_packages.contains(&extra_pkg) {
                    result.remove(&extra_pkg);
                }
            }
        }

        // If a package is installed as an extra of a parent package,
        // and the parent package is not used, then the extra package is
        // annotated as an extra package of the (possibly) unused parent package
        for (pkg, extras) in self.extra_packages.iter() {
            for extra in extras.iter() {
                let extra_pkg = reformat_package_name(&extra);
                if unused_packages.contains(pkg) && unused_packages.contains(&extra_pkg) {
                    result.remove(&extra_pkg);
                    let annotated_pkg = format!("{} - an extra of {}", extra, pkg);
                    result.insert(annotated_pkg);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unused_packages() {
        let project_path: PathBuf = PathBuf::from("tests/fixtures/example_project");
        let verbose = false;
        let dev = false;
        let project = PythonProject::new(project_path, verbose, dev);
        let result = project.get_unused_packages();
        let expected = [
            "tenacity".to_string(),
            "sentry_sdk".to_string(),
            "redis".to_string(),
            "scikit_learn".to_string(),
        ];
        assert_eq!(result, expected.iter().cloned().collect());
    }

    #[test]
    fn test_find_unused_manifest_packages() {
        let project_path: PathBuf = PathBuf::from("tests/fixtures/example_project");
        let verbose = false;
        let dev = false;
        let project = PythonProject::new(project_path, verbose, dev);
        let result = project.find_unused_manifest_packages();
        let expected = [
            "tenacity".to_string(),
            "dotenv".to_string(),
            "email_validator".to_string(),
            "sentry_sdk".to_string(),
            "redis".to_string(),
            "scikit_learn".to_string(),
        ];
        assert_eq!(result, expected.iter().cloned().collect());
    }

    #[test]
    fn test_filter_package_extras() {
        let project_path: PathBuf = PathBuf::from("tests/fixtures/example_project");
        let verbose = false;
        let dev = false;
        let project = PythonProject::new(project_path, verbose, dev);
        let unused_packages = project.get_unused_packages();
        let result = project.filter_package_extras(unused_packages);
        let expected = [
            "tenacity".to_string(),
            "sentry_sdk".to_string(),
            "redis".to_string(),
            "scikit_learn".to_string(),
        ];
        assert_eq!(result, expected.iter().cloned().collect());
    }

    #[test]
    fn test_filter_package_extras_returns_annotated_package() {
        let project_path: PathBuf = PathBuf::from("tests/fixtures/example_project_2");
        let verbose = false;
        let dev = false;
        let project = PythonProject::new(project_path, verbose, dev);
        let unused_packages = project.get_unused_packages();
        let result = project.filter_package_extras(unused_packages);
        let expected = [
            "requests".to_string(),
            "pydantic".to_string(),
            "email-validator - an extra of pydantic".to_string(),
        ];
        assert_eq!(result, expected.iter().cloned().collect());
    }
}