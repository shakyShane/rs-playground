use Flag;
use bool::bool_from;

pub fn get_dry(user_input: &Vec<&str>) -> Flag<bool> {
    Flag {
        value: bool_from(&user_input, &vec!["dry"]).unwrap_or(false),
        name: "dry".into(),
        description: "Shortcut for setting the run mode".into()
    }
}