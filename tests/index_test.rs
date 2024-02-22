use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;


#[test]
/// Test info subcommand
///
/// Input: gfa
/// Output: pi (index)
fn index_pack() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-g")
        .arg("data/example/9986.100k.gfa")
        .arg("-o")
        .arg("data/test/9986.100k.pi");
    cmd1.unwrap().assert().success();
    Ok(())
}


#[test]
/// Test info subcommand
///
/// Input: pack
/// Output: pi (index)
fn index_gfa() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-g")
        .arg("data/example/testGraph_complex.gfa")
        .arg("-o")
        .arg("data/test/testGraph_complex.test.pi");
    cmd1.unwrap().assert().success();
    Ok(())
}

