use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packing")?;
    cmd.arg("index")
        .arg("-g")
        .arg("dadsadasd")
        .arg("-o")
        .arg("test");
    cmd.assert().stderr(predicate::str::contains("No file"));

    Ok(())
}
