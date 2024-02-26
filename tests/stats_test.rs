use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn stats_p() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("stats")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.1k.stats.txt");
    cmd1.unwrap().assert().success();

    let contents = fs::read_to_string("data/test/9986.1k.stats.txt").expect("Unable to read file");
    assert_eq!(contents.contains("Average (with zeros) 1"), true);





    Ok(())
}

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn stats_p2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("stats")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("-");
    cmd1.unwrap().assert().success();

    cmd1.assert().stdout(predicate::str::contains("Average (with zeros) 1"));




    Ok(())
}

