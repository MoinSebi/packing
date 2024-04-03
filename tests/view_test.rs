use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use std::fs;

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn view_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.sequence.a1.pt")
        .arg("-v");
    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("view")
        .arg("-c")
        .arg("data/test/9986.sequence.a1.pt")
        .arg("-o")
        .arg("data/test/9986.1k.view.txt");
    cmd1.unwrap().assert().success();

    let contents = fs::read_to_string("data/test/9986.1k.view.txt").expect("Unable to read file");
    assert!(contents.contains("1\n1\n0\n1\n1"));
    Ok(())
}

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn view_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.sequence.pc");

    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("view")
        .arg("-c")
        .arg("data/test/9986.sequence.pc")
        .arg("-o")
        .arg("data/test/9986.1k.view2.txt");
    cmd1.unwrap().assert().success();

    let contents = fs::read_to_string("data/test/9986.1k.view2.txt").expect("Unable to read file");
    assert!(contents.contains("4\n2\n0\n3\n6"));
    fs::remove_file("data/test/9986.1k.view2.txt")?;
    fs::remove_file("data/test/9986.sequence.pc")?;

    Ok(())
}
