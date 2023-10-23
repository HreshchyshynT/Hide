use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*;
use serde_json::Value; // Used for writing assertions
use std::{fs, io::Write, process::Command}; // Run programs

static STRING_PLACEHOLDER: &str = "String";
static NUMBER_PLACEHOLDER: &str = "Number";
static BOOL_PLACEHOLDER: &str = "Bool";

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
        .arg("--add-keys")
        .arg("name,surname,age");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "\"name\": \"{}\"",
            STRING_PLACEHOLDER,
        )))
        .stdout(predicate::str::contains(format!(
            "\"surname\": \"{}\"",
            STRING_PLACEHOLDER,
        )))
        .stdout(predicate::str::contains(format!(
            "\"age\": \"{}\"",
            NUMBER_PLACEHOLDER,
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
        STRING_PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();
    file.write_str(input)?;

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-keys")
        .arg("password")
        .arg("--remove-keys")
        .arg("user");

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
        STRING_PLACEHOLDER, STRING_PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();

    file.write_str(input)?;

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-keys")
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
    cmd.arg("--add-keys").arg("name,surname");
    cmd.unwrap().assert().success();

    assert!(file_path.exists());

    let file = fs::read_to_string(&file_path).unwrap();
    assert!(file.contains("'name'"));
    assert!(file.contains("'surname'"));

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("--remove-keys").arg("name");
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
fn test_object_values_inside_array() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let array_inside_object = r#"{
    "users": [
        {"name": "Alice", "password": "secret1", "age": 44, "is_owner": true },
        {"name": "Bob", "password": "secret2", "age": 45, "is_owner": false }
    ]
}"#;
    file.write_str(array_inside_object).unwrap();
    let expected_output = format!(
        r#"
    {{
        "users": [
            {{"name": "Alice", "password": "{}", "age": "{}", "is_owner": "{}" }},
            {{"name": "Bob", "password": "{}", "age": "{}", "is_owner": "{}" }}
        ]
    }}
    "#,
        STRING_PLACEHOLDER,
        NUMBER_PLACEHOLDER,
        BOOL_PLACEHOLDER,
        STRING_PLACEHOLDER,
        NUMBER_PLACEHOLDER,
        BOOL_PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-keys")
        .arg("password,age,is_owner")
        .arg("--remove-keys")
        .arg("name,users"); // ensure that key 'name' is not in "to hide" list

    // TODO: learn why do we need to_owned() here
    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output: Value = serde_json::from_str(&String::from_utf8(output)?)?;
    assert_eq!(expected_output, output);
    Ok(())
}

#[test]
fn test_empty_json_object() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let array_inside_object = r#"{
    "users": {}
}"#;
    file.write_str(array_inside_object).unwrap();
    let expected_output = r#"
    {
        "users": {}
    }
    "#;
    let expected_output: Value = serde_json::from_str(expected_output).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i")
        .arg(file.path())
        .arg("--add-keys")
        .arg("users");

    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output: Value = serde_json::from_str(&String::from_utf8(output)?)?;
    assert_eq!(expected_output, output);
    Ok(())
}

#[test]
fn test_hide_values_in_json_object() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let array_inside_object = r#"
        {
            "user": {
                "name": "Jon",
                "age": 45
            }
        }"#;
    file.write_str(array_inside_object).unwrap();
    let expected_output = format!(
        r#"{{
        "user": {{
            "name": "{}",
            "age": "{}"
        }}
    }}"#,
        STRING_PLACEHOLDER, NUMBER_PLACEHOLDER,
    );
    let expected_output: Value = serde_json::from_str(&expected_output).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i").arg(file.path()).arg("--add-keys").arg("user");

    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output: Value = serde_json::from_str(&String::from_utf8(output)?)?;
    assert_eq!(expected_output, output);
    Ok(())
}

#[test]
fn hide_null() -> Result {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    let input = r#"{"key": null}"#;
    file.write_str(input).unwrap();
    let expected_output = r#"{"key": null}"#;
    let expected_output: Value = serde_json::from_str(expected_output).unwrap();

    let mut cmd = Command::cargo_bin("hide")?;
    cmd.arg("-i").arg(file.path()).arg("--add-keys").arg("key");

    let output = cmd.assert().success().get_output().stdout.to_owned();
    let output: Value = serde_json::from_str(&String::from_utf8(output)?)?;
    assert_eq!(expected_output, output);
    Ok(())
}
