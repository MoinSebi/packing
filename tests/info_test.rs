use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
/// Test info subcommand
///
/// Input: gfa
/// Output: pi (index)
fn info_pb() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info").arg("-c").arg("data/example/9986.1k.pb");
    cmd1.assert()
        .stderr(predicate::str::contains("Entry type: Node"));
    cmd1.unwrap().assert().success();

    Ok(())
}

#[test]
fn info_gfa() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info").arg("-i").arg("data/example/9986.1k.pi");
    cmd1.assert()
        .stderr(predicate::str::contains("Number of nodes: 67"));
    cmd1.assert()
        .stderr(predicate::str::contains("Number of entries: 999"));
    cmd1.unwrap().assert().success();
    Ok(())
}

#[test]
fn info_pt_bin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info")
        .arg("-c")
        .arg("data/example/9986.1k.a1.bin.pt");
    cmd1.unwrap().assert().success();
    Ok(())
}

#[test]
fn info_pt_u16() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info").arg("-c").arg("data/example/9986.1k.a1.pt");
    cmd1.unwrap().assert().success();
    Ok(())
}
