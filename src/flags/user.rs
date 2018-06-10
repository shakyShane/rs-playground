use Flag;
use string::string_from;

pub fn get_user(user_input: &Vec<&str>) -> Flag<String> {
    Flag {
        value: string_from(&user_input, &vec!["user"]).unwrap_or("www-data".into()),
        name: "user".into(),
        description: "the user under which to run commands".into()
    }
}