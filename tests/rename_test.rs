use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn rename1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.rename.pc");
    cmd.assert().success();

    let mut cmd2 = Command::cargo_bin("packing")?;
    cmd2.arg("rename")
        .arg("-i")
        .arg("data/test/packing.rename.pc")
        .arg("-n")
        .arg("test321313")
        .arg("-o")
        .arg("data/test/packing.rename.rename.pc");
    cmd2.unwrap().assert().success();

    let mut cmd3 = Command::cargo_bin("packing")?;

    cmd3.arg("info")
        .arg("-c")
        .arg("data/test/data/test/packing.rename.rename.pc");
    cmd3.unwrap().assert().success();
    cmd3.assert().stderr(predicate::str::contains("test321313"));

    //fs::remove_file("data/test/9986.1k.sequence.rename.pc")?;
    //fs::remove_file("data/test/9986.sequence.pc")?;

    Ok(())
}
