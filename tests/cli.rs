use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*;
use serde_json::json; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hide")?;

    cmd.arg("-i").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn hide_values() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let json = json!({
        "name": "Name",
        "surname": "Surname",
        "age": 99
    });
    file.write_str(&json.to_string())?;

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-words")
        .arg("name,surname");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"hidden\""))
        .stdout(predicate::str::contains("\"surname\": \"hidden\""));

    Ok(())
}

#[test]
fn words_saved_to_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("--add-words").arg("name,surname");
    let file_path = confy::get_configuration_file_path("hide", "hide-cfg").unwrap();
    assert!(file_path.exists());
    let file = std::fs::read_to_string(file_path).unwrap();
    assert!(file.contains("one"));
    assert!(file.contains("two"));
    Ok(())
}
