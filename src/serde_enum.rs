use std::collections::HashMap;
use from_file::FromFile;

#[derive(Debug, Deserialize, FromFile)]
struct Input {
    tasks: HashMap<String, TaskDef>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum TaskDef {
    Task { command: String, env: Option<Vec<String>> },
    CmdString(String),
    TaskSeq(Vec<TaskDef>),
}

pub fn main() {
    match Input::from_file("./src/static/input.yaml") {
        Ok(s) => {
            match s.tasks.get("client") {
                Some(TaskDef::CmdString(command)) => {
                    assert_eq!(*command, "rimraf dist && webpack --color --env.mode production".to_string());
                }
                _ => unreachable!()
            }
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}