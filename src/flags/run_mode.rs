use Flag;
use RunMode;
use string::string_from;

pub fn get_run_mode(user_input: &Vec<&str>) -> Flag<RunMode> {

    let keys = &vec!["run_mode", "runmode", "run-mode", "runMode"];
    let default = "execute";
    let value = match string_from(&user_input, keys).unwrap_or(default.into()).as_ref() {
        "dryrun" | "dry-run" | "dry_run" | "dryRun" => RunMode::DryRun,
        _ => RunMode::Execute,
    };

    Flag {
        value,
        name: "run_mode".into(),
        description: "the run mode, can be either execute or dry-run".into(),
    }
}

