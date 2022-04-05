

use assert_cmd::Command;

#[test]
fn index_gfa() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("index")
        .arg("-g")
        .arg("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa")
        .arg("-o")
        .arg("tests_output/t1.pi");
    cmd.assert().success();
    Ok(())
}


#[test]
fn index_pack() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("index")
        .arg("-p")
        .arg("9986.100k.txt")
        .arg("-o")
        .arg("tests_output/t2.pi");
    cmd.assert().success();

    Ok(())
}

