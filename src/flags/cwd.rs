use Flag;
use string::string_from;
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_cwd(user_input: &Vec<&str>) -> Flag<PathBuf> {
    Flag {
        value: match string_from(&user_input, &vec!["cwd"]) {
            Some(t) => PathBuf::from(t),
            None => current_dir().unwrap()
        },
        name: "cwd".into(),
        description: "path to run commands from".into(),
    }
}