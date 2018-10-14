struct Pet {
    name: String
}

impl Into<String> for Pet {
    fn into(self) -> String {
        self.name
    }
}

impl<'a> From<&'a str> for Pet {
    fn from(n: &str) -> Pet {
        Pet {
            name: n.into(),
        }
    }
}

struct Person {
    name: String
}

impl From<Person> for String {
    fn from(p: Person) -> String {
        p.name
    }
}

pub fn run() {
    let my_str = "hello";
    let my_string = "hello2".to_string();

    // printing 2 different types
    test_fn(my_str);
    test_fn(my_string);

    // printing 2 different structs, that both implement Into<String>
    test_fn(Person{name: "shane".into()});
    test_fn(Pet{name: "kittie".into()});

    // conversions from &str -> Pet with .into()
    let cat: Pet = "freddie".into();
    test_fn(cat);

    // conversions from &str -> Pet with ::from()
    let cat = Pet::from("freddie");
    test_fn(cat);
}

fn test_fn(input: impl Into<String>) {
    println!("printed={}", input.into());
}
