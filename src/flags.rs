use FlagValue;
use Flag;
use string::string_from;
use RunMode;
use std::env::current_dir;
use bool::bool_from;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProgramFlags {
    pub cwd: FlagValue<PathBuf>,
    pub dry: FlagValue<bool>,
    pub quiet: FlagValue<bool>,
    pub user: FlagValue<String>,
    pub run_mode: FlagValue<RunMode>,
}

pub fn create_program_flags<'a>(user_input: &Vec<&'a str>) -> ProgramFlags {
    ProgramFlags {
        cwd: FlagValue::new(Flag {
            value: match string_from(&user_input, &vec!["cwd"]).unwrap_or("NA".into()).as_ref() {
                "NA" => current_dir().unwrap(),
                _a => PathBuf::from(_a)
            },
            name: "cwd".into(),
            description: "path to run commands from".into()
        }),
        quiet: FlagValue::new(Flag {
            value: bool_from(&user_input, &vec!["quiet", "q"]).unwrap_or(false),
            name: "quiet".into(),
            description: "silence the output".into()
        }),
        dry: FlagValue::new(Flag {
            value: bool_from(&user_input, &vec!["dry"]).unwrap_or(false),
            name: "dry".into(),
            description: "Shortcut for setting the run mode".into()
        }),
        user: FlagValue::new(Flag {
            value: string_from(&user_input, &vec!["user"]).unwrap_or("www-data".into()),
            name: "user".into(),
            description: "the user under which to run commands".into()
        }),
        run_mode: FlagValue::new(Flag {
            value: match string_from(&user_input, &vec!["run_mode", "runmode", "run-mode", "runMode"]).unwrap_or("execute".into()).as_ref() {
                "dryrun" | "dry-run" | "dry_run" | "dryRun" => RunMode::DryRun,
                _ => RunMode::Execute,
            },
            name: "run_mode".into(),
            description: "the run mode, can be either execute or dry-run".into()
        })
    }
}

pub fn post_process(program_flags: &mut ProgramFlags) -> &mut ProgramFlags {
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