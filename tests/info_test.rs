use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use std::fs;
use std::io::Stderr;
use std::process::{Command as c2, Stdio};
use assert_cmd::cargo::CommandCargoExt;

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn info_v1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.info.9986.node.a2.pc")
        .arg("--node")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();


    let mut output = c2::cargo_bin("packing")?
        .arg("info").arg("-c").arg("data/test/packing.info.9986.node.a2.pc").stderr(Stdio::piped()).stdout(Stdio::piped()).output()?;

    let as1 = String::from_utf8(output.stderr).unwrap();
    assert!(as1.contains("Bytes: 9\n"));
    Ok(())
}

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn info_v2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.info.9986.node.a2.pn")
        .arg("--node")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();


    let mut output = c2::cargo_bin("packing")?
        .arg("info").arg("-c").arg("data/test/packing.info.9986.node.a2.pn").stderr(Stdio::piped()).stdout(Stdio::piped()).output()?;

    let as1 = String::from_utf8(output.stderr).unwrap();
    assert!(as1.contains("Entries: 67\n"));
    Ok(())
}

