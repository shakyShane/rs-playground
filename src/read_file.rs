extern crate serde_yaml;

use std::path::PathBuf;
use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: usize,
}

enum ConfigType {
    Invalid,
    File(String),
}

pub fn run() {
    let input = String::from("file:tests/fixtures/conf.yaml");

    let conf = get_config_type(&input)
        .and_then(|maybe_path| {
            let mut file = File::open(maybe_path).map_err("oops");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("another");
            Ok(contents)
        })
        .and_then(|contents: String| {
            println!("file={}", contents);
            Ok(contents)
        })
        .expect("oh no");

    println!("conf={}", conf);
}

fn get_config_type(input: &str) -> Result<String, String> {
    let split: Vec<&str> = input.split(":").collect();
    match split.len() {
        2 => Ok(split[1].into()),
        _ => Err("Nope".to_string())
    }
}