use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*;
use serde_json::Value; // Used for writing assertions
use std::{fs, io::Write, process::Command}; // Run programs

static PLACEHOLDER: &str = "[hidden]";

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn file_doesnt_exist() -> Result {
    let mut cmd = Command::cargo_bin("hide")?;

    cmd.arg("-i").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn hide_values_in_simple_json() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let json: Value = serde_json::from_str(
        r#"{
        "name": "Name",
        "surname": "Surname",
        "age": 99
    }"#,
    )
    .unwrap();

    file.write_str(&json.to_string())?;

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-words")
        .arg("name,surname");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "\"name\": \"{}\"",
            PLACEHOLDER,
        )))
        .stdout(predicate::str::contains(format!(
            "\"surname\": \"{}\"",
            PLACEHOLDER,
        )));

    Ok(())
}

#[test]
fn hide_values_in_nested_json() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let input = r#"
        {
            "user": {
                "username": "alice",
                "credentials": {
                    "password": "secret"
                }
            }
        }
        "#;

    let expected_output = format!(
        r#"{{
        "user": {{
            "username": "alice",
            "credentials": {{
                "password": "{}"
            }}
        }}
    }}"#,
        PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();
    file.write_str(input)?;

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-words")
        .arg("password");

    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output_json: Value = serde_json::from_str(&String::from_utf8(output).unwrap()).unwrap();
    assert_eq!(expected_output, output_json);
    Ok(())
}

#[test]
fn hide_values_in_json_array() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let input = r#"
    [
        {"username": "alice", "password": "secret1"},
        {"username": "bob", "password": "secret2"}
    ]
    "#;

    let expected_output = format!(
        r#"
    [
        {{"username": "alice", "password": "{}"}},
        {{"username": "bob", "password": "{}"}}
    ]
    "#,
        PLACEHOLDER, PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();

    file.write_str(input)?;

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-words")
        .arg("password");

    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output = String::from_utf8(output).unwrap();
    let output: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(expected_output, output);
    Ok(())
}

// test is flaky if running with default "cargo test [--test cli]" because of all tests run in parallel
// to ensure all works well use "cargo test --test cli -- --test-threads=1"
#[test]
fn test_storing_config() -> Result {
    let file_path = confy::get_configuration_file_path("hide", "hide-cfg").unwrap();
    if file_path.exists() {
        fs::remove_file(&file_path).unwrap();
    }

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("--add-words").arg("name,surname");
    cmd.unwrap().assert().success();

    assert!(file_path.exists());

    let file = fs::read_to_string(&file_path).unwrap();
    assert!(file.contains("'name'"));
    assert!(file.contains("'surname'"));

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("--remove-words").arg("name");
    cmd.assert().success();

    let file = fs::read_to_string(&file_path).unwrap();
    println!("file: {file}");
    std::io::stdout().flush().unwrap();
    assert_eq!(file.contains("'name'"), false);
    assert!(file.contains("'surname'"));

    Ok(())
}

#[test]
fn test_empty_object() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let empty_object = r#"{}"#;
    file.write_str(&empty_object)?;
    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("{}"));
    Ok(())
}

#[test]
fn test_empty_array() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let empty_array = r#"[]"#;
    file.write_str(&empty_array)?;
    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[]"));
    Ok(())
}

#[test]
fn test_invalid_input_json() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    // Missing closing brace for the entire JSON object
    let invalid_input = r#"{
        "user": {
            "username": "alice",
            "credentials": {
                "password": "secret",
            } 
    }"#;
    file.write_str(invalid_input).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i").arg(file.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not parse file"));
    Ok(())
}

#[test]
fn test_array_inside_object() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let array_inside_object = r#"{
    "users": [
        {"name": "Alice", "password": "secret1"},
        {"name": "Bob", "password": "secret2"}
    ]
}"#;
    file.write_str(array_inside_object).unwrap();
    let expected_output = format!(
        r#"
    {{
        "users": [
            {{"name": "Alice", "password": "{}"}},
            {{"name": "Bob", "password": "{}"}}
        ]
    }}
    "#,
        PLACEHOLDER, PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-words")
        .arg("password")
        .arg("--remove-words")
        .arg("name, users"); // ensure that key 'name' is not in "to hide" list

    // TODO: learn why do we need to_owned() here
    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output: Value = serde_json::from_str(&String::from_utf8(output)?)?;
    assert_eq!(expected_output, output);
    Ok(())
}

#[test]
fn test_hide_json_object() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let array_inside_object = r#"{
    "users": [
        {"name": "Alice", "password": "secret1"},
        {"name": "Bob", "password": "secret2"}
    ]
}"#;
    file.write_str(array_inside_object).unwrap();
    let expected_output = format!(
        r#"
    {{
        "users": "{}"
    }}
    "#,
        PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-words")
        .arg("users");

    // TODO: learn why do we need to_owned() here
    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output: Value = serde_json::from_str(&String::from_utf8(output)?)?;
    assert_eq!(expected_output, output);
    Ok(())
}
