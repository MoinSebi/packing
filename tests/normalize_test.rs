use assert_cmd::Command;
use packing_lib::core::reader::unpack_zstd_to_byte;
use std::fs;

//--------------------------------------------------------------------------------------
#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pn
/// Type: sequence
/// Threshold: 1
fn normalize_sequence_a1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.sequence.default.pn")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.sequence.default.pn");
    assert_eq!(a.len(), 999 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.sequence.default.pn")?;
    Ok(())
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pn
/// Type: sequence
/// Threshold: 1
fn normalize_sequence_a3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.sequence.a3.pn")
        .arg("-a")
        .arg("3")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.sequence.a3.pn");
    assert_eq!(a.len(), 999 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.sequence.a3.pn")?;

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
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.sequence.median.f050.pn")
        .arg("-f")
        .arg("0.50")
        .arg("-m")
        .arg("median")
        .arg("-v");
    cmd.assert().success();
    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.sequence.median.f050.pn");
    assert_eq!(a.len(), 999 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.sequence.median.f050.pn")?;
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
fn normalize_sequence_r50() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.sequence.a2.pn")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.sequence.a2.pn");
    assert_eq!(a.len(), 999 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.sequence.a2.pn")?;

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

//--------------------------------------------------------------------------------------
#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pn
/// Type: sequence
/// Threshold: 1
fn normalize_node_a1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.node.a1.pn")
        .arg("--node")
        .arg("-v");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.node.a1.pn");
    assert_eq!(a.len(), 67 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.node.a1.pn")?;

    Ok(())
}

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pn
/// Type: sequence
/// Threshold: 1
fn normalize_node_a3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.node.a3.pn")
        .arg("--node")
        .arg("-a")
        .arg("3")
        .arg("-v");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.node.a3.pn");
    assert_eq!(a.len(), 67 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.node.a3.pn")?;

    Ok(())
}

#[test]
/// Test normalize subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// -s (stats) median
/// -b (binary) (compress)
fn normalize_node_median() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.node.median.f050.pn")
        .arg("--node")
        .arg("-f")
        .arg("0.5")
        .arg("-m")
        .arg("median")
        .arg("-v");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.node.median.f050.pn");
    assert_eq!(a.len(), 67 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.node.median.f050.pn")?;

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
fn normalize_node_r50() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.node.a2.pn")
        .arg("--node")
        .arg("-a")
        .arg("2")
        .arg("-v");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.node.a2.pn");
    assert_eq!(a.len(), 67 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.node.a2.pn")?;

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}

#[test]
/// Test normalize subcommand with
/// -t (type) sequences
/// -r (relative threshold) 50
/// --normalize (u16)
fn normalize_nodes_norm() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("normalize")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/packing.normalize.9986.node.norm.f050.pn")
        .arg("-f")
        .arg("0.50")
        .arg("-m")
        .arg("percentile")
        .arg("-v")
        .arg("--node");
    cmd.assert().success();

    let a = unpack_zstd_to_byte("data/test/packing.normalize.9986.node.norm.f050.pn");
    assert_eq!(a.len(), 67 * 4 + 86);
    fs::remove_file("data/test/packing.normalize.9986.node.norm.f050.pn")?;

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}
