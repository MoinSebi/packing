use std::fs;
use assert_cmd::Command;
use packing_lib::core::reader::{unpack_zstd_to_byte, zstd_decode};

//--------------------------------------------------------------------------------------
#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn bit_sequence_a1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.sequence.default.pt")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.sequence.default.pt");
    assert_eq!(a.len(), 125 + 86);
    fs::remove_file("data/test/packing.bit.9986.sequence.default.pt")?;

    Ok(())
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn bit_sequence_a3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.sequence.a3.pt")
        .arg("-a")
        .arg("3")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.sequence.a3.pt");
    assert_eq!(a.len(), 125 + 86);
    fs::remove_file("data/test/packing.bit.9986.sequence.a3.pt")?;

    Ok(())
}

#[test]
/// Test normalize subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// -s (stats) median
/// -b (binary) (compress)
fn convert_pack_sequence_median() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.sequence.median.f050.pt")
        .arg("-f")
        .arg("0.50")
        .arg("-m")
        .arg("median")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.sequence.median.f050.pt");
    assert_eq!(a.len(), 125 + 86);
    fs::remove_file("data/test/packing.bit.9986.sequence.median.f050.pt")?;

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn bit_sequence_r50() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.sequence.a2.pt")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.sequence.a2.pt");
    assert_eq!(a.len(), 125 + 86);
    fs::remove_file("data/test/packing.bit.9986.sequence.a2.pt")?;

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

//--------------------------------------------------------------------------------------
#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn bit_node_a1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.node.default.pt")
        .arg("--node")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.node.default.pt");
    assert_eq!(a.len(), 9 + 86);
    fs::remove_file("data/test/packing.bit.9986.node.default.pt")?;

    Ok(())
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pt
/// Type: sequence
/// Threshold: 1
fn bit_node_a3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.node.a3.pt")
        .arg("--node")
        .arg("-a")
        .arg("3")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.node.a3.pt");
    assert_eq!(a.len(), 9 + 86);
    fs::remove_file("data/test/packing.bit.9986.node.a3.pt")?;

    Ok(())
}

#[test]
/// Test normalize subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// -s (stats) median
/// -b (binary) (compress)
fn bit_node_median() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.node.median.f050.pt")
        .arg("--node")
        .arg("-f")
        .arg("0.5")
        .arg("-m")
        .arg("median")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.node.median.f050.pt");
    assert_eq!(a.len(), 9 + 86);
    fs::remove_file("data/test/packing.bit.9986.node.median.f050.pt")?;

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn bit_node_r50() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.node.a2.pt")
        .arg("--node")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.node.a2.pt");
    assert_eq!(a.len(), 9 + 86);
    fs::remove_file("data/test/packing.bit.9986.node.a2.pt")?;


    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

#[test]
/// Test normalize subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// --normalize (u16)
fn bit_nodes_norm() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("bit")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.bit.9986.node.norm.f050.pt")
        .arg("-f")
        .arg("0.50")
        .arg("-m")
        .arg("percentile")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.bit.9986.node.norm.f050.pt");
    // 999/8 = 124.875
    assert_eq!(a.len(), 125 + 86);

    fs::remove_file("data/test/packing.bit.9986.node.norm.f050.pt")?;


    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}
