use assert_cmd::Command;

#[test]
fn runs_with_success() {
    let mut cmd = Command::cargo_bin("deptr").unwrap();
    cmd.arg("./tests/fixtures/");
    let output = cmd.unwrap();
    assert!(output.status.success());
}

#[test]
fn runs_with_expected_error_no_pyproject_file() {
    let mut cmd = Command::cargo_bin("deptr").unwrap();
    cmd.assert().failure();
    cmd.assert().code(1);
    cmd.assert()
        .stderr("Error: Unable to find a pyproject.toml file\n");
}

#[test]
fn runs_with_expected_error_invalid_path() {
    let mut cmd = Command::cargo_bin("deptr").unwrap();
    cmd.arg("bad/path");
    cmd.assert().failure();
    cmd.assert().code(1);
    cmd.assert().stderr("Error: Invalid path provided\n");
}
