use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use std::fs;

#[test]
/// Test view
///
/// Input: pack
/// Output: pt
///
fn view_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.view.9986.sequence.a1.pt")
        .arg("-v")
        .arg("-a")
        .arg("1");

    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.view.9986.1k.pi");
    cmd1.unwrap().assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("view")
        .arg("-c")
        .arg("data/test/packing.view.9986.sequence.a1.pt")
        .arg("-o")
        .arg("data/test/packing.view.9986.1k.view.txt")
        .arg("-i")
        .arg("data/test/packing.view.9986.1k.pi");
    cmd1.unwrap().assert().success();

    let contents =
        fs::read_to_string("data/test/packing.view.9986.1k.view.txt").expect("Unable to read file");
    assert!(contents.contains("354\t28\t0\t1"));
    fs::remove_file("data/test/packing.view.9986.sequence.a1.pt")?;
    fs::remove_file("data/test/packing.view.9986.1k.view.txt")?;
    fs::remove_file("data/test/packing.view.9986.1k.pi")?;

    Ok(())
}

#[test]
/// Test view subcommand
///
/// Input: pack
/// Output: pn
fn view_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.view.9986.sequence.pn")
        .arg("-a")
        .arg("3");
    cmd.assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.view.9986.1k2.pi");
    cmd1.unwrap().assert().success();

    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("view")
        .arg("-c")
        .arg("data/test/packing.view.9986.sequence.pn")
        .arg("-o")
        .arg("data/test/9986.1k.view2.txt")
        .arg("-i")
        .arg("data/test/packing.view.9986.1k2.pi");
    cmd1.unwrap().assert().success();

    let _contents = fs::read_to_string("data/test/9986.1k.view2.txt").expect("Unable to read file");
    //assert!(contents.contains("4\n2\n0\n3\n6"));
    fs::remove_file("data/test/9986.1k.view2.txt")?;
    fs::remove_file("data/test/packing.view.9986.sequence.pn")?;
    fs::remove_file("data/test/packing.view.9986.1k2.pi")?;

    Ok(())
}
