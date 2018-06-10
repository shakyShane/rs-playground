use std::fmt;
use flags::create_program_flags;
use flags::post_process;

mod string;
mod bool;
mod flags;

#[derive(Debug)]
pub struct FlagValue<T> {
    pub inner: Flag<T>
}

impl <T> FlagValue<T> {
    fn new(x: Flag<T>) -> FlagValue<T> {
        FlagValue { inner: x }
    }
    fn value(&self) -> &T {
        &self.inner.value
    }
}

impl <T> fmt::Display for FlagValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
           f,
           "{}: {}",
           self.inner.name,
           self.inner.description
        )
    }
}

#[derive(Debug)]
pub struct Flag<T> {
    pub value: T,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub enum RunMode {
    Execute,
    DryRun
}


fn main() {
//    let flags = create_program_flags();
//    println!("{:#?}", flags.user);
}

#[test]
fn test_main() {

    let mut flags = create_program_flags(&vec!["--run_mode", "dryRun", "-q", "--cwd", "/users/kittiens"]);

    match flags {
        Ok(program_flags) => println!("{:#?}", program_flags),
        Err(msg) => eprintln!("error msg = {}", msg)
    }

//    // just pulling the values in a type-safe way
//    assert_eq!(*flags.user.value(), "www-data".to_string());
//    assert_eq!(*flags.quiet.value(), true);
//
//    // ability to match on the type
//    match flags.user.value().as_ref() {
//        "root" => println!("Nope, not running as root"),
//        _a =>  println!("running as {}", _a)
//    }
//
//    match *flags.run_mode.value() {
//        RunMode::Execute => println!("run mode is execute"),
//        RunMode::DryRun => println!("run mode is dryrun"),
//    }
//
//    assert_eq!(*flags.run_mode.value(), RunMode::DryRun);
//
//
//    let processed = post_process(&mut flags);
//
//    println!("{:#?}", processed);
}