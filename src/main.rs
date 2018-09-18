#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
extern crate serde;

mod dyn_arg;
mod compose;
mod trait_obj;
mod serialize;
mod matching_tuple;

fn main() {
    dyn_arg::run();
    compose::run();
    trait_obj::run();
    serialize::run();
    matching_tuple::run();
}
