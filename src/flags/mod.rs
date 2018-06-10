use FlagValue;
use RunMode;
use std::path::PathBuf;
use flags::run_mode::get_run_mode;
use flags::user::get_user;
use flags::dry::get_dry;
use flags::quiet::get_quiet;
use flags::cwd::get_cwd;
use std::error::Error;

mod run_mode;
mod user;
mod dry;
mod quiet;
mod cwd;

#[derive(Debug)]
pub struct ProgramFlags {
    pub cwd: FlagValue<PathBuf>,
    pub dry: FlagValue<bool>,
    pub quiet: FlagValue<bool>,
    pub user: FlagValue<String>,
    pub run_mode: FlagValue<RunMode>,
}

pub fn create_program_flags<'a>(user_input: &Vec<&'a str>) -> Result<ProgramFlags, String> {
    Ok(ProgramFlags {
        cwd: FlagValue::new(get_cwd(&user_input)?),
        quiet: FlagValue::new(get_quiet(&user_input)?),
        dry: FlagValue::new(get_dry(&user_input)?),
        user: FlagValue::new(get_user(&user_input)?),
        run_mode: FlagValue::new(get_run_mode(&user_input)?)
    })
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
    let mut flags = create_program_flags(&vec!["--dry", "--cwd", "/users/shakyshane"]).unwrap();
    let processed = post_process(&mut flags);
    assert_eq!(*processed.run_mode.value(), RunMode::DryRun);
    assert_eq!(*processed.cwd.value(), PathBuf::from("/users/shakyshane"));

    println!("exist? = {}", processed.cwd.value().exists());
}