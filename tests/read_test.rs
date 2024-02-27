use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use std::fs;
use packing_lib::core::reader::{unpack_zstd_to_byte, wrapper_bool};

#[test]
/// Test stats
///
/// Input: gfa
/// Output: pi (index)
fn view_1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = unpack_zstd_to_byte("test.t.pa");
    let a = wrapper_bool(&bytes);

    assert_eq!(a.len(), 2);
    assert_eq!(a[0].length, 999);
    assert_eq!(a[0].bin_coverage.len(), 999);
    assert_eq!(a[1].bin_coverage.len(), 999);

    Ok(())
}