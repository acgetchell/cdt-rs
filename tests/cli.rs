use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn exit_success() {
    let mut cmd = Command::cargo_bin("cdt-rs").unwrap();
    cmd.assert().success();
}

#[test]
fn cdt_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cdt-rs")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Point"));

    Ok(())
}
