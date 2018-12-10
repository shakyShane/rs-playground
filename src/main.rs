#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate serde;
extern crate void;
extern crate from_file;

#[macro_use]
extern crate from_file_derive;

mod dyn_arg;
mod fold;
mod compose;
mod trait_obj;
mod serialize;
mod matching_tuple;
mod serde_yaml_parse;
mod hash_map;
mod read_file;
mod into;
mod collect_collections;
mod serde_enum;

fn main() {
    dyn_arg::main();
    compose::main();
    trait_obj::main();
    serialize::main();
    matching_tuple::main();
    serde_yaml_parse::main();
    hash_map::main();
    read_file::main();
    into::main();
    fold::main();
    collect_collections::main();
    serde_enum::main();
}
