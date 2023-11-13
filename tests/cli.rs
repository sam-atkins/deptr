use assert_cmd::Command;

#[test]
fn runs_with_success() {
    let mut cmd = Command::cargo_bin("deptr").unwrap();
    let output = cmd.unwrap();
    assert!(output.status.success());
}
