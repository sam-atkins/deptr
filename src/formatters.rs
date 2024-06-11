//! Formatters used to change package names to aid in matching manifest packages with import statements.

/// Reformats the package name to improve the likelihood of matching the import statement
/// - Some packages are named `python-something` but imported as `something` so we strip
///   `python-` from the name
/// - Some packages have dashes but are imported using underscores so we replace `-` with `_`
pub fn reformat_package_name(package: &str) -> String {
    let mut package = package.to_string();
    if package.starts_with("python-") {
        package = package[7..].to_string();
    }
    if package.contains('-') {
        package = package.replace('-', "_");
    }
    package
}

#[test]
fn test_reformat_package_name() {
    let dep = "python-redis";
    let transformed_dep = reformat_package_name(dep);
    assert_eq!(transformed_dep, "redis");

    let dep = "python-redis-abc";
    let transformed_dep = reformat_package_name(dep);
    assert_eq!(transformed_dep, "redis_abc");

    let dep = "redis-abc";
    let transformed_dep = reformat_package_name(dep);
    assert_eq!(transformed_dep, "redis_abc");

    let dep = "redis";
    let transformed_dep = reformat_package_name(dep);
    assert_eq!(transformed_dep, "redis");
}
