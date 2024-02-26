use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

#[test]
/// Test info subcommand
///
/// Input: pack
/// Output: pi (index)
fn index_pack() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.1k.pi");
    cmd1.unwrap().assert().success();

    let mut cmd2 = Command::cargo_bin("packing")?;
    cmd2.arg("info").arg("-i").arg("data/test/9986.1k.pi");
    cmd2.assert()
        .stderr(predicate::str::contains("Number of nodes: 67"));
    cmd2.assert()
        .stderr(predicate::str::contains("Number of entries: 999"));
    cmd2.unwrap().assert().success();
    fs::remove_file("data/test/9986.1k.pi")?;



    Ok(())
}

#[test]
/// Test info subcommand
///
/// Input: gfa
/// Output: pi (index)
fn index_gfa() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-g")
        .arg("data/example/testGraph2.gfa")
        .arg("-o")
        .arg("data/test/testGraph_complex.test.pi");
    cmd1.unwrap().assert().success();

    let mut cmd2 = Command::cargo_bin("packing")?;
    cmd2.arg("info").arg("-i").arg("data/test/testGraph_complex.test.pi");
    cmd2.assert()
        .stderr(predicate::str::contains("Number of nodes: 9"));
    cmd2.unwrap().assert().success();
    fs::remove_file("data/test/testGraph_complex.test.pi")?;

    Ok(())
}
