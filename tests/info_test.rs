use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn index_pack() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("packing")?;
    cmd1.arg("index")
        .arg("-g")
        .arg("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa")
        .arg("-o")
        .arg("tests_output/t10");
    cmd1.unwrap().assert().success();

    let mut cmd2 = Command::cargo_bin("packing")?;
    cmd2.arg("info")
        .arg("-i")
        .arg("tests_output/t10.bin.zst");
    cmd2.assert().stdout(predicate::str::contains("Number of nodes: 9"));
    cmd2.assert().stdout(predicate::str::contains("Number of entries: 58"));


    Ok(())
}

#[test]
fn index_pack2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("index")
        .arg("-p")
        .arg("9986.100k.txt")
        .arg("-o")
        .arg("tests_output/t20");
    cmd.assert().success();

    let mut cmd2 = Command::cargo_bin("packing")?;
    cmd2.arg("info")
        .arg("-i")
        .arg("tests_output/t20.bin.zst");
    cmd2.assert().stdout(predicate::str::contains("Number of nodes: 7404"));
    cmd2.assert().stdout(predicate::str::contains("Number of entries: 99999"));


    Ok(())
}


#[test]
fn index_pack3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("9986.100k.txt")
        .arg("-o")
        .arg("tests_output/t21");
    cmd.assert().success();

    let mut cmd2 = Command::cargo_bin("packing")?;
    cmd2.arg("info")
        .arg("-b")
        .arg("tests_output/t21.bin.zst");
    cmd2.assert().stdout(predicate::str::contains("7404"));


    Ok(())
}