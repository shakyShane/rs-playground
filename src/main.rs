extern crate regex;

use regex::Regex;

use std::borrow::Cow;


fn main() {
    let subject = "abc";

    let fns: Vec<fn(Cow<str>) -> Cow<str>> = vec![replace_a, replace_b];

    let folded = fns.iter()
        .fold(Cow::Owned(String::from(subject)), |acc, fn_item| fn_item(acc));

    assert_eq!(folded, "ABc")
}

fn replace_a(input: Cow<str>) -> Cow<str> {
    match input {
        Cow::Borrowed(b) => Regex::new("a").unwrap().replace_all(b, "A"),
        Cow::Owned(o) => Cow::Owned(Regex::new("a").unwrap().replace_all(&o, "A").into_owned()),
    }
}

fn replace_b(input: Cow<str>) -> Cow<str> {
    match input {
        Cow::Borrowed(b) => Regex::new("b").unwrap().replace_all(b, "B"),
        Cow::Owned(o) => Cow::Owned(Regex::new("b").unwrap().replace_all(&o, "B").into_owned()),
    }
}
