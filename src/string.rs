pub fn string_from<'a>(user_input: &Vec<&'a str>, names: &Vec<&'a str>) -> Option<String> {
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
                    Some(value) => Some(value.to_string()),
                    None => None
                }
            },
            None => None
        }
}

#[test]
fn test_string_from() {
    let user_input: Vec<&str> = vec!["--user", "shane"];
    assert_eq!(string_from(&user_input, &vec!["user"]), Some("shane".to_string()));

    let user_input: Vec<&str> = vec!["--user"];
    assert_eq!(string_from(&user_input, &vec!["user"]), None);

    let user_input: Vec<&str> = vec![];
    assert_eq!(string_from(&user_input, &vec!["user"]), None);
//
    let user_input: Vec<&str> = vec!["--runmode", "dry"];
    assert_eq!(string_from(&user_input, &vec!["run_mode", "runmode"]), Some("dry".to_string()));
}