extern crate serde_yaml;

use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

#[derive(Eq, PartialEq, Debug, Deserialize)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug)]
enum ConfigType {
    Invalid,
    File(String),
}

#[derive(Debug)]
enum ConfigError {
    InvalidInput,
    FileOpen,
    FileRead,
    SerdeError(serde_yaml::Error)
}

pub fn run() {

    let input = String::from("file:tests/fixtures/conf.yaml");
    let conf = get_config(&input);
    assert_eq!(conf.unwrap(), Person{
        name: "shane".into(),
        age: 34,
    });

    //
    // This example will fail to deserialize to a Person
    // since the `name` field should be a String
    //
    let invalid_person = r#"name: [1, 2]"#;
    let result = parse_from_string(invalid_person.to_string());
    match result {
        Err(config_error) => match config_error {
            ConfigError::SerdeError(e) => {
                let output = format!("{}", e);
                assert_eq!(output, "name: invalid type: sequence, expected a string at line 1 column 7")
            }
            _ => println!("unknown")
        },
        Ok(p) => println!("person={:?}", p),
    };
}

///
/// From a string like `file:config.yaml`, try to read the file
/// and if it exists, parse into a strongly typed struct `Person`
///
fn get_config(input: &str) -> Result<Person, ConfigError> {
    get_file_path(&input)
        .and_then(read_from_path)
        .and_then(parse_from_string)
}

///
/// Parse strings like file:config.yaml to extract the file path only
///
fn get_file_path(input: &str) -> Result<String, ConfigError> {
    let split: Vec<&str> = input.split(":").collect();
    match split.len() {
        2 => Ok(split[1].into()),
        _ => Err(ConfigError::InvalidInput)
    }
}

///
/// Take a user-given path & try to read the file from disk into a String
///
fn read_from_path(maybe_path: String) -> Result<String, ConfigError> {
    let mut file = File::open(maybe_path).map_err(|_| ConfigError::FileOpen)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| ConfigError::FileRead)?;
    Ok(contents)
}

///
/// Parse any YAML string directly into a Person Struct
///
fn parse_from_string(contents: String) -> Result<Person, ConfigError> {
    serde_yaml::from_str(&contents).map_err(|e| ConfigError::SerdeError(e))
}


