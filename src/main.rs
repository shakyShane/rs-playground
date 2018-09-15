#![allow(unused_variables)]
#![allow(dead_code)]

mod dyn_arg;
mod compose;
mod trait_obj;

fn main() {
    dyn_arg::run();
    compose::run();
    trait_obj::run();
}