use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
/// Test normalize subcommand with
///
/// Input: pack
/// Output: pb
/// Type: sequence
/// Modifier:
///     - absolute threshold: 2
fn compare() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("compare")
        .arg("--compressed1")
        .arg("data/example/9986.1k.a1.pt")
        .arg("--compressed2")
        .arg("data/example/9986.1k.a2.pt");
    cmd.assert().success();
    cmd.assert().stderr(predicate::str::contains("Real threshold is different"));

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
fn compare2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("compare")
        .arg("--compressed1")
        .arg("data/example/9986.1k.a1.pt")
        .arg("--compressed2")
        .arg("data/example/9986.1k.a1.pt");
    cmd.assert().success();
    cmd.assert().stderr(predicate::str::contains("Meta data is the same"));

    Ok(())
    //    cmd.assert().stdout(predicate::str::contains("Number of entries: 99999"));
}