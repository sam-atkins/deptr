use assert_cmd::Command;
use predicates::prelude::*;

use deptr::validators::PathError::{InvalidPath, MissingPyprojectToml, NonSupportedTooling};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const APP: &str = "deptr";

#[test]
fn runs_with_success() -> TestResult {
    Command::cargo_bin(APP)?
        .arg("./tests/fixtures/")
        .assert()
        .success();
    Ok(())
}

#[test]
fn runs_with_expected_error_no_pyproject_file() -> TestResult {
    Command::cargo_bin(APP)?
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(MissingPyprojectToml));
    Ok(())
}

#[test]
fn runs_with_expected_error_invalid_path() -> TestResult {
    Command::cargo_bin(APP)?
        .arg("bad/path")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(InvalidPath));
    Ok(())
}

#[test]
fn runs_with_expected_error_not_poetry_project() -> TestResult {
    Command::cargo_bin(APP)?
        .arg("tests/fixtures/input/non_poetry")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(NonSupportedTooling));
    Ok(())
}
