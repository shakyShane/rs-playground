#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate serde;

mod dyn_arg;
mod compose;
mod trait_obj;
mod serialize;
mod matching_tuple;
mod serde_yaml_parse;
mod hash_map;
mod read_file;

fn main() {
    dyn_arg::run();
    compose::run();
    trait_obj::run();
    serialize::run();
    matching_tuple::run();
    serde_yaml_parse::run();
    hash_map::run();
    read_file::run();
}
