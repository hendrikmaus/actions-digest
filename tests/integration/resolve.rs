use assert_cmd::prelude::*;
use std::fs::read_to_string;
use std::process::Command;

const BASE_DIR: &str = "tests/integration";

#[test]
fn process_workflow() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let before = format!("{}/data/before.yaml", BASE_DIR);
    let after = read_to_string(&format!("{}/data/after.yaml", BASE_DIR))?;

    cmd.arg(&before).assert().success().stdout(after);

    Ok(())
}
