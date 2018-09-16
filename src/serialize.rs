extern crate serde_json;
extern crate serde_derive;

use std::sync::Mutex;

struct MyStruct<'a> {
    items: Vec<&'a str>
}

pub fn run() {

    //
    // Use a Mutex to 'wrap' this data in a way that allows
    // reads + writes across threads
    //
    let my_struct = Mutex::new(
        MyStruct {
            items: vec!["a", "b"]
        }
    );

    //
    // Imagine this is on some other thread, you can gain access to the struct
    // by 'locking' access to it.
    //
    let unwrapped = my_struct.lock().unwrap();

    //
    // Now we can read the data, but to allow serde_json to serialize
    // it, we have to use `*` to follow the pointer to the raw data,
    // and then prefix it with `&` since 'to_string' here expects a
    // reference
    //
    let json = serde_json::to_string(&*unwrapped.items).unwrap();

    assert_eq!(json, r#"["a","b"]"#);
}
