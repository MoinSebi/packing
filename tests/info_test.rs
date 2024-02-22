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
    cmd1.arg("info")
        .arg("-b")
        .arg("data/example/9986.1k.pb");
    cmd1.assert().stdout(predicate::str::contains("Number of elements: 1"));
    cmd1.unwrap().assert().success();

    Ok(())
}

fn info_gfa() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info")
        .arg("-pi")
        .arg("data/example/9986.100k.pi");
    cmd1.assert().stdout(predicate::str::contains("Number of nodes: 67"));
    cmd1.assert().stdout(predicate::str::contains("Number of entries: 999"));
    cmd1.unwrap().assert().success();
    Ok(())
}



fn info_pt_bin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info")
        .arg("-p")
        .arg("data/example/9986.1k.a1.bin.pt");
    cmd1.unwrap().assert().success();
    Ok(())
}

fn info_pt_u16() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info")
        .arg("-p")
        .arg("data/example/9986.1k.a1.pt");
    cmd1.unwrap().assert().success();

    Ok(())
}


