use assert_cmd::Command;
use packing_lib::reader::{get_file_as_byte_vec, wrapper_u16};

#[test]
fn convert_pack_normal() -> Result<(), Box<dyn std::error::Error>> {
    // How to test convert
    // If zstd --> read it and check one of two numbers
    // make 1000k example
    // Number of entries
    // check some nubers
    // compute by hand

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("9986.100k.txt")
        .arg("-o")
        .arg("tests_output/t21");
    cmd.assert().success();

    let o = get_file_as_byte_vec("tests_output/t21.bin.zst");
    let p = wrapper_u16(&o);
    println!("{}", p[0].name);
    println!("{}dsalkdlsajkdljaskd", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[1], 0 );
    assert_eq!(p[0].data[49], 1);
    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("could not read file"));


    Ok(())
}

#[test]
fn convert_pack_nodes() -> Result<(), Box<dyn std::error::Error>> {
    // How to test convert
    // If zstd --> read it and check one of two numbers
    // make 1000k example
    // Number of entries
    // check some nubers
    // compute by hand

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("9986.100k.txt")
        .arg("-o")
        .arg("tests_output/t21");
    cmd.assert().success();

    Ok(())
}