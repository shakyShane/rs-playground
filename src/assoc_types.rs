use regex::Regex;

///
/// A generic updater, it takes self, an options struct
/// and a collection of things to apply.
///
/// In my case I'll be using fn's, but this definition
/// does not enforce that.
///
trait Updater {
    type Item;
    type Options;
    fn apply(self, opts: &Self::Options, items: Vec<Self::Item>) -> String;
}

///
/// Just a struct for holding any options that might be
/// needed in the 'apply' method.
///
struct MyOpts {
    target: String,
    next: String
}

///
/// A simple wrapper struct for a String.
/// This will be the 'subject'
///
struct Subject(String);

///
/// Now we can implement the Updater trait
/// for our subject. We need to define the associated
/// types Item & Options as declared in the Updater trait.
///
/// This is how we create generic functionality.
///
impl Updater for Subject {

    // I want each Self::Item to be a fn of a certain signature
    type Item = fn(&str, &MyOpts) -> String;

    // And the options is just MyOpts
    type Options = MyOpts;

    ///
    /// Now I can implement the apply method, in my case a fold
    /// passing the original string value (self.0) through every fn in
    /// sequence producing a single string output.
    ///
    fn apply(self, opts: &Self::Options, fns: Vec<Self::Item>) -> String {
        fns.iter().fold(String::from(self.0), |acc, fn_item| fn_item(&acc, &opts))
    }
}

fn run() {

    // simple options
    let opts = MyOpts {
        target: String::from("acme.de"),
        next: String::from("127.0.0.1")
    };

    // Create the struct holding a string, and pass along
    // the options and the Vec of fns
    let actual = Subject(String::from("https://acme.de"))
        .apply(&opts, vec![
            replace_host,
            replace_scheme,
        ]);

    assert_eq!(actual, "//127.0.0.1")
}

///
/// Now we can create as many of these small, isolated fns
/// that are easy to understand, easy to re-use and
/// easy to test and we can dynamically choose which ones
/// are applied at run time.
///
fn replace_host(input: &str, opts: &MyOpts) -> String {
    Regex::new(&opts.target).unwrap().replace_all(&input, opts.next.as_str()).to_string()
}

fn replace_scheme(input: &str, _opts: &MyOpts) -> String {
    Regex::new("https://").unwrap().replace_all(&input, "//").to_string()
}
