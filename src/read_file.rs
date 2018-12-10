extern crate serde_yaml;

use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use from_file::FromFile;

#[derive(Eq, PartialEq, Debug, Deserialize)]
struct Person {
    name: String,
    age: usize,
}

impl FromFile for Person {}

pub fn main() {

    let p2 = Person::from_yml_file("tests/fixtures/conf.yaml");
    assert_eq!(p2.unwrap(), Person{
        name: "shane".into(),
        age: 34,
    });

    let p2 = Person::from_json_file("tests/fixtures/person.json");
    assert_eq!(p2.unwrap(), Person{
        name: "kittie".into(),
        age: 10,
    });
}


