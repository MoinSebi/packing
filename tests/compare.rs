use std::fs;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn compare_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.compare1.9986.node.a1.pc")
        .arg("--node")
        .arg("-a")
        .arg("1")
        .arg("-v");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.compare1.9986.node.a2.pc")
        .arg("--node")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();


    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("compare")
        .arg("--compressed1")
        .arg("data/test/packing.compare1.9986.node.a1.pc")
        .arg("--compressed2")
        .arg("data/test/packing.compare1.9986.node.a2.pc");
    cmd.assert().success();
    cmd.assert()
        .stderr(predicate::str::contains("Real threshold is different"));

    fs::remove_file("data/test/packing.compare1.9986.node.a1.pc")?;
    fs::remove_file("data/test/packing.compare1.9986.node.a2.pc")?;


    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn compare_same() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.compare2.9986.node.a2.pc")
        .arg("--node")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();



    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("compare")
        .arg("--compressed1")
        .arg("data/test/packing.compare2.9986.node.a2.pc")
        .arg("--compressed2")
        .arg("data/test/packing.compare2.9986.node.a2.pc");
    cmd.assert().success();
    cmd.assert()
        .stderr(predicate::str::contains("Meta data is the same"));

    fs::remove_file("data/test/packing.compare2.9986.node.a2.pc")?;



    Ok(())

    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}
