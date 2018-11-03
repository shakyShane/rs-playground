pub fn main() {
    let match1 = matcher(1, "120");
    let match2 = matcher(1, "240");
    let match3 = matcher(1, "360");
    let match4 = matcher(2, "400");

    assert_eq!(match1, 1);
    assert_eq!(match2, 2);
    assert_eq!(match3, 2);
    assert_eq!(match4, 0);
}

///
/// An example of matching values as pairs using a
/// temporary tuple
///
fn matcher(first: usize, second: &str) -> usize {
    match (first, second) {
        (1, "120") => 1,
        (1, "240") | (1, "360") => 2,
        (_first, _second) => 0
    }
}
