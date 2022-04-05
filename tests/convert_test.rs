use assert_cmd::Command;
use predicates::prelude::predicate;
use packing_lib::reader::{get_file_as_byte_vec, wrapper_bool, wrapper_u16};

#[test]
fn convert_pack_nothing() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg("tests_output/t21")
        .arg("-v");
    cmd.assert().success();

    let o = get_file_as_byte_vec("tests_output/t21.bin.zst");
    let p = wrapper_u16(&o);
    cmd.assert().stderr(predicate::str::contains("File is"));
    println!("{}", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[1], 0 );
    assert_eq!(p[0].data[49], 1);
    assert_eq!(p[0].data.len(), 7404);


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
        .arg("tests_output/t22")
        .arg("-t")
        .arg("sequence")
        .arg("-v");
    cmd.assert().success();
    let o = get_file_as_byte_vec("tests_output/t22.bin.zst");
    let p = wrapper_u16(&o);
    println!("{}", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[374], 7 );
    assert_eq!(p[0].data[2732], 11);
    assert_eq!(p[0].data.len(), 99999);


    Ok(())
}


#[test]
fn convert_pack_nodes_a() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg("tests_output/t23")
        .arg("-t")
        .arg("sequence")
        .arg("-a")
        .arg("2")
        .arg("-b")
        .arg("-v");
    cmd.assert().success();
    let o = get_file_as_byte_vec("tests_output/t23.bin.zst");
    let p = wrapper_bool(&o);
    println!("{}", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[374], true );
    assert_eq!(p[0].data[2732], true);
    assert_eq!(p[0].data[117], false);
    assert_eq!(p[0].data[106], false);


    // check this
    assert_eq!(p[0].data.len(), 100_000);


    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}



#[test]
fn convert_pack_nodes_r() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg("tests_output/t23")
        .arg("-t")
        .arg("sequence")
        .arg("-r")
        .arg("50")
        .arg("-b")
        .arg("-v");
    cmd.assert().success();
    let o = get_file_as_byte_vec("tests_output/t23.bin.zst");
    let p = wrapper_bool(&o);
    // cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
    println!("{}", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[181], false );
    assert_eq!(p[0].data[2732], true);
    assert_eq!(p[0].data[117], false);
    assert_eq!(p[0].data[106], false);


    // check this (because one byte more)
    assert_eq!(p[0].data.len(), 100_000);


    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}


#[test]
fn convert_pack_nodes_median() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg("tests_output/t23")
        .arg("-t")
        .arg("sequence")
        .arg("-r")
        .arg("50")
        .arg("-b")
        .arg("-s")
        .arg("median")
        .arg("-v");
    cmd.assert().success();
    let o = get_file_as_byte_vec("tests_output/t23.bin.zst");
    let p = wrapper_bool(&o);
    println!("{}", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[495], true );
    assert_eq!(p[0].data[2732], true);
    assert_eq!(p[0].data[117], false);
    assert_eq!(p[0].data[106], false);


    // check this (because one byte more)
    assert_eq!(p[0].data.len(), 100_000);


    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}


#[test]
fn convert_pack_nodes_norm() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg("tests_output/t23")
        .arg("-t")
        .arg("sequence")
        .arg("-r")
        .arg("50")
        .arg("--normalize")
        .arg("-v");
    cmd.assert().success();
    let o = get_file_as_byte_vec("tests_output/t23.bin.zst");
    let p = wrapper_u16(&o);
    //cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

    println!("{}", p[0].name);
    assert_eq!(p[0].name, "9986.100k.txt");
    assert_eq!(p[0].data[677], 0);
    assert_eq!(p[0].data[678], 1);
    assert_eq!(p[0].data[2103], 2);

    // check this (because one byte more)
    assert_eq!(p[0].data.len(), 99_999);


    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}