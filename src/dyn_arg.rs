///
/// CHALLENGE:
///
/// Implement a function that can take 2 parameters:
///     1. param 1 = &str
///     2. param 2 = &str OR closure
///
/// The idea is to learn how to replicate such amazing
/// flexibility as that seen in crates like Regex where they have:
///
///     Regex::new("abc").unwrap().replace_all("a", "A")
///     OR
///     Regex::new("abc").unwrap().replace_all("a", |caps: &Captures| "A")
///

///
/// Define the base method signature for a 'replacer'
///
pub trait Replacer {
    fn replacer_fn(&mut self, input: &str) -> String;
}

///
/// Implementing Replacer for a &str means that &mut self
/// here is the actual string slice
///
impl<'a> Replacer for &'a str {
    fn replacer_fn(&mut self, _input: &str) -> String {
        self.to_string()
    }
}

///
/// Just a simple struct to hold a string slice with a lifetime,
/// this will be given to a given closure as it's only parameter
///
pub struct Matches<'a> {
    text: &'a str
}

///
/// Now we implement Replacer for a FnMut - this is what will
/// allow us to accept a closure OR a &str
///
impl<F> Replacer for F where F: FnMut(&Matches) -> String {
    fn replacer_fn(&mut self, input: &str) -> String {
        self(&Matches {text: input})
    }
}

///
/// Empty struct onto which we can implement methods
///
pub struct Regex;

///
/// This is our what allows this public API:
///     Regex::replace("a", "B")
///     OR
///     Regex::replace("a", |_input: &Matches| String::from("A"))
///
impl Regex {
    pub fn replace<R>(input: &str, mut output: R) -> String where R: Replacer {
        output.replacer_fn(input)
    }
}

pub fn run() {
    let r1 = Regex::replace("a", "A");
    let r2 = Regex::replace("b", |input: &Matches| input.text.to_uppercase());

    assert_eq!(r1, "A".to_string());
    assert_eq!(r2, "B".to_string());
}