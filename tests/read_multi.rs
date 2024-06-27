use assert_cmd::Command;
use packing_lib::core::core::PackCompact;
use packing_lib::core::reader::{unpack_zstd_to_byte, wrapper_reader};

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn normalize_sequence_r50() -> Result<(), Box<dyn std::error::Error>> {
    let f2 = unpack_zstd_to_byte("data/example/9986.1k.a2.copy.pt");
    let f = wrapper_reader(&f2);
    assert_eq!(f.len(), 2);
    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}