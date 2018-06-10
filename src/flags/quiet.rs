use Flag;
use bool::bool_from;

pub fn get_quiet(user_input: &Vec<&str>) -> Flag<bool> {
    Flag {
        value: bool_from(&user_input, &vec!["quiet", "q"]).unwrap_or(false),
        name: "quiet".into(),
        description: "silence the output".into()
    }
}