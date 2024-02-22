use assert_cmd::Command;
use packing_lib::reader::{get_file_as_byte_vec, wrapper_bool, wrapper_meta, wrapper_u16};
use packing_lib::vg_parser::parse_smart;
use predicates::prelude::predicate;

#[test]
/// Test convert
///
/// - Input: pack
/// - Output: pb
fn convert_pb() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("convert")
        .arg("-p")
        .arg("data/example/9986.1k.txt")
        .arg("-o")
        .arg("data/test/pack_binary.pc")
        .arg("-v");
    cmd.assert().success();

    Ok(())
}
