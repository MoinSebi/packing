use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn info_v1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info")
        .arg("-c")
        .arg("data/example/9986.1k.pn");
    cmd1.unwrap().assert().success();

    let _contents = fs::read_to_string("data/test/9986.1k.stats.txt").expect("Unable to read file");

    Ok(())
}

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn info_v2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("info")
        .arg("-c")
        .arg("data/example/9986.1k.pn");
    cmd1.unwrap().assert().success();



    Ok(())
}
