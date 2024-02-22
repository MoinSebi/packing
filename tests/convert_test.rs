use assert_cmd::Command;
use predicates::prelude::predicate;
use packing_lib::reader::{get_file_as_byte_vec, wrapper_bool, wrapper_meta, wrapper_u16};
use packing_lib::vg_parser::parse_smart;


//--------------------------------------------------------------------------------------
#[test]
/// Test convert subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn convert_pt_sequence_a1() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.sequence.a1.pt")
        .arg("-t")
        .arg("sequence")
        .arg("--binary")
        .arg("-v");
    cmd.assert().success();
    let o = get_file_as_byte_vec("./data/test/9986.sequence.a1.pt");
    let p = wrapper_bool(&o);

    Ok(())
}

#[test]
/// Test convert subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn convert_pt_sequence_a3() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.100k.txt")
        .arg("-o")
        .arg("data/test/9986.sequence.a1.pt")
        .arg("-t")
        .arg("sequence")
        .arg("-a")
        .arg("3")
        .arg("-v");
    cmd.assert().success();


    Ok(())
}

#[test]
/// Test convert subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// -s (stats) median
/// -b (binary) (bit)
fn convert_pack_sequence_median() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.100k.txt")
        .arg("-o")
        .arg("data/test/9986.sequence.a3.pt")
        .arg("-t")
        .arg("sequence")
        .arg("-r")
        .arg("50")
        .arg("-b")
        .arg("-s")
        .arg("median")
        .arg("-v");
    cmd.assert().success();


    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}




#[test]
/// Test convert subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn convert_pt_sequence_r50() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.sequence.r50.pt")
        .arg("-t")
        .arg("sequence")
        .arg("-a")
        .arg("2")
        .arg("-b")
        .arg("-v");
    cmd.assert().success();


    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}



//--------------------------------------------------------------------------------------
#[test]
/// Test convert subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn convert_pt_node_a1() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/9986.node.a1.pt")
        .arg("-t")
        .arg("node")
        .arg("-v");
    cmd.assert().success();


    Ok(())
}

#[test]
/// Test convert subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn convert_pt_node_a3() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.100k.txt")
        .arg("-o")
        .arg("data/test/9986.node.a3.pb")
        .arg("-t")
        .arg("node")
        .arg("-a")
        .arg("3")
        .arg("-v");
    cmd.assert().success();


    Ok(())
}

#[test]
/// Test convert subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// -s (stats) median
/// -b (binary) (bit)
fn convert_pt_node_median() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.100k.txt")
        .arg("-o")
        .arg("data/test/9986.node.median.pt")
        .arg("-t")
        .arg("node")
        .arg("-r")
        .arg("50")
        .arg("-b")
        .arg("-s")
        .arg("median")
        .arg("-v");
    cmd.assert().success();



    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}




#[test]
/// Test convert subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn convert_pt_node_r50() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.100k.txt")
        .arg("-o")
        .arg("data/test/9986.node.r50.pt")
        .arg("-t")
        .arg("node")
        .arg("-a")
        .arg("2")
        .arg("-b")
        .arg("-v");
    cmd.assert().success();



    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}




#[test]
/// Test convert subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// --normalize (u16)
fn convert_pt_nodes_norm() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.100k.txt")
        .arg("-o")
        .arg("data/test/9986.node.norm.r50.pt")
        .arg("-t")
        .arg("sequence")
        .arg("-r")
        .arg("50")
        .arg("--normalize")
        .arg("-v");
    cmd.assert().success();
    Ok(())
//    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));

}

