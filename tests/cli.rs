use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn exit_success() {
    let mut cmd = Command::cargo_bin("cdt-rs").unwrap();
    cmd.arg("32");
    cmd.assert().success();
}

#[test]
fn cdt_cli_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cdt-rs")?;

    cmd.arg("32");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Point"));

    Ok(())
}

#[test]
fn cdt_cli_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cdt-rs")?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "error: the following required arguments were not provided:",
    ));

    Ok(())
}
