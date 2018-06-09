use std::fmt;
use std::path::PathBuf;
use std::env::current_dir;

#[derive(Debug)]
struct FlagValue<T> {
    inner: Flag<T>
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
struct Flag<T> {
    value: T,
    name: String,
    description: String,
}

#[derive(Debug, PartialEq)]
enum RunMode {
    Execute,
    DryRun
}

#[derive(Debug)]
struct ProgramFlags {
    cwd: FlagValue<PathBuf>,
    dry: FlagValue<bool>,
    quiet: FlagValue<bool>,
    user: FlagValue<String>,
    run_mode: FlagValue<RunMode>,
}

fn create_program_flags<'a>(user_input: &Vec<&'a str>) -> ProgramFlags {
    ProgramFlags {
        cwd: FlagValue::new(Flag {
            value: match string_from(&user_input, &vec!["cwd"], "NA").as_ref() {
                "NA" => current_dir().unwrap(),
                _a => PathBuf::from(_a)
            },
            name: "cwd".into(),
            description: "path to run commands from".into()
        }),
        quiet: FlagValue::new(Flag {
            value: bool_from(&user_input, &vec!["quiet", "q"], false),
            name: "quiet".into(),
            description: "silence the output".into()
        }),
        dry: FlagValue::new(Flag {
            value: bool_from(&user_input, &vec!["dry"], false),
            name: "dry".into(),
            description: "Shortcut for setting the run mode".into()
        }),
        user: FlagValue::new(Flag {
            value: string_from(&user_input, &vec!["user"], "www-data"),
            name: "user".into(),
            description: "the user under which to run commands".into()
        }),
        run_mode: FlagValue::new(Flag {
            value: match string_from(&user_input, &vec!["run_mode", "runmode", "run-mode", "runMode"], "execute").as_ref() {
                "dryrun" | "dry-run" | "dry_run" | "dryRun" => RunMode::DryRun,
                _ => RunMode::Execute,
            },
            name: "run_mode".into(),
            description: "the run mode, can be either execute or dry-run".into()
        })
    }
}

fn post_process(program_flags: &mut ProgramFlags) -> &mut ProgramFlags {
    match *program_flags.dry.value() {
        true => program_flags.run_mode.inner.value = RunMode::DryRun,
        _ => (),
    }

    program_flags
}

#[test]
fn test_post_process() {
    let mut flags = create_program_flags(&vec!["--dry", "--cwd", "/users/shakyshane"]);
    let processed = post_process(&mut flags);
    assert_eq!(*processed.run_mode.value(), RunMode::DryRun);
    assert_eq!(*processed.cwd.value(), PathBuf::from("/users/shakyshane"));

    println!("exist? = {}", processed.cwd.value().exists());
}

fn bool_from<'a>(user_input: &Vec<&'a str>, names: &Vec<&'a str>, def: bool) -> bool {
    match user_input.iter().find(|x| {
        let short = names.iter().find(|name| format!("-{}", name) == x.as_ref()).map(|_x| true);
        let long = names.iter().find(|name| format!("--{}", name) == x.as_ref()).map(|_x| true);

        return short.unwrap_or(long.unwrap_or(false));
    }) {
        Some(_t) => true,
        None => def
    }
}

#[test]
fn test_bool_from() {
    let user_input: Vec<&str> = vec!["--quiet"];
    assert_eq!(bool_from(&user_input, &vec!["quiet"], false), true);

    let user_input: Vec<&str> = vec![];
    assert_eq!(bool_from(&user_input, &vec!["quiet"], false), false);

    let user_input: Vec<&str> = vec!["-q"];
    assert_eq!(bool_from(&user_input, &vec!["quiet", "q"], false), true);
}

fn string_from<'a>(user_input: &Vec<&'a str>, names: &Vec<&'a str>, def: &str) -> String {
    let len = user_input.len();
    let indexes = 0..len;

    match indexes.zip(user_input)
        .find(|&(_index, x)| {
            match names.iter().find(|n| format!("--{}", n) == x.as_ref()) {
                Some(_t) => true,
                None => false
            }
        })
        .map(|(index, _x)| user_input.get(index + 1))
        {
        Some(t) => {
            match t {
                Some(value) => value.to_string(),
                None => String::from(def)
            }
        },
        None => String::from(def)
    }
}

#[test]
fn test_string_from() {
    let user_input: Vec<&str> = vec!["--user", "shane"];
    assert_eq!(string_from(&user_input, &vec!["user"], "www-data"), "shane");

    let user_input: Vec<&str> = vec!["--user"];
    assert_eq!(string_from(&user_input, &vec!["user"], "www-data"), "www-data");

    let user_input: Vec<&str> = vec![];
    assert_eq!(string_from(&user_input, &vec!["user"], "www-data"), "www-data");

    let user_input: Vec<&str> = vec!["--runmode", "dry"];
    assert_eq!(string_from(&user_input, &vec!["run_mode", "runmode"], "exec"), "dry");
}

#[test]
fn test_assign_user_flags() {
    let user_input: Vec<&str> = vec!["--user", "shane"];
    let mut _program_flags = create_program_flags(&user_input);

}

fn main() {
//    let flags = create_program_flags();
//    println!("{:#?}", flags.user);
}

#[test]
fn test_main() {

    let mut flags = create_program_flags(&vec!["--run_mode", "dryRun", "-q"]);

    // just pulling the values in a type-safe way
    assert_eq!(*flags.user.value(), "www-data".to_string());
    assert_eq!(*flags.quiet.value(), true);

    // ability to match on the type
    match flags.user.value().as_ref() {
        "root" => println!("Nope, not running as root"),
        _a =>  println!("running as {}", _a)
    }

    match *flags.run_mode.value() {
        RunMode::Execute => println!("run mode is execute"),
        RunMode::DryRun => println!("run mode is dryrun"),
    }

    assert_eq!(*flags.run_mode.value(), RunMode::DryRun);

    println!("{:#?}", flags);

    let processes = post_process(&mut flags);
}