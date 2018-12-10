pub fn main() {
    let mut has_a = vec![];

    let words = vec!["a", "b", "c", "abcd"];

    words.into_iter().for_each(|w| {
        if w.contains("a") {
            has_a.push(format!("==:{}", w));
        }
    });

    let words = vec!["a", "b", "c", "abcd"];
    let has_a: Vec<String> = words.into_iter().filter_map(extracted).collect();
}

fn extracted(w: &str) -> Option<String> {
    if w.contains("a") {
        Some(format!("==:{}", w))
    } else {
        None
    }
}