use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use std::fs;

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn stats_p() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.stats.pc");
    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("stats")
        .arg("-c")
        .arg("data/test/packing.stats.pc")
        .arg("-o")
        .arg("data/test/packing.stats.pc,stats");
    cmd1.unwrap().assert().success();

    let _contents =
        fs::read_to_string("data/test/packing.stats.pc,stats").expect("Unable to read file");

    Ok(())
}
#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn stats_p3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.stats.pc");
    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("stats")
        .arg("-c")
        .arg("data/test/packing.stats.pc")
        .arg("-o")
        .arg("data/test/packing.stats.pc,stats");
    cmd1.unwrap().assert().success();

    let _contents =
        fs::read_to_string("data/test/packing.stats.pc,stats").expect("Unable to read file");

    Ok(())
}

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn stats_p4() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.stats.pc");
    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("stats")
        .arg("-c")
        .arg("data/test/packing.stats.pc")
        .arg("-o")
        .arg("data/test/packing.stats.pc,stats");
    cmd1.unwrap().assert().success();

    let _contents =
        fs::read_to_string("data/test/packing.stats.pc,stats").expect("Unable to read file");

    Ok(())
}

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn stats_p2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.stats.pc");
    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("stats")
        .arg("-c")
        .arg("data/test/packing.stats.pc")
        .arg("-o")
        .arg("data/test/packing.stats.pc,stats");
    cmd1.unwrap().assert().success();

    let _contents =
        fs::read_to_string("data/test/packing.stats.pc,stats").expect("Unable to read file");

    Ok(())
}
