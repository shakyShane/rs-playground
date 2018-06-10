use Flag;
use string::string_from;
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_cwd(user_input: &Vec<&str>) -> Result<Flag<PathBuf>, String> {

    let value = match string_from(&user_input, &vec!["cwd"]) {
        Some(t) => PathBuf::from(t),
        None => current_dir().unwrap()
    };

    if !value.exists() {
        return Err(format!("Could not use {:?} as cwd, path did not exist", value));
    }

    Ok(Flag {
        value,
        name: "cwd".into(),
        description: "path to run commands from".into(),
    })
}