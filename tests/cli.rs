use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs_with_success() -> TestResult {
    Command::cargo_bin("deptr")?
        .arg("./tests/fixtures/")
        .assert()
        .success();
    Ok(())
}

#[test]
fn runs_with_expected_error_no_pyproject_file() -> TestResult {
    Command::cargo_bin("deptr")?
        .assert()
        .failure()
        .code(1)
        .stderr("Error: Unable to find a pyproject.toml file\n");
    Ok(())
}

#[test]
fn runs_with_expected_error_invalid_path() -> TestResult {
    Command::cargo_bin("deptr")?
        .arg("bad/path")
        .assert()
        .failure()
        .code(1)
        .stderr("Error: Invalid path provided\n");
    Ok(())
}
