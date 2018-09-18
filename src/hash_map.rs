use std::collections::HashMap;

trait Speak {
    fn greet(&self) -> String;
}

#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Animal {
    name: String
}

impl Speak for Person { fn greet(&self) -> String { format!("person={}", self.name.to_string()) }}
impl Speak for Animal { fn greet(&self) -> String { format!("animal={}", self.name.to_string()) }}

pub fn run() {
    let mut m: HashMap<usize, Box<Speak>> = HashMap::new();

    let items = vec!["person", "animal", "person"];

    items.iter().enumerate().for_each(|(index, t)| {
        match t.as_ref() {
            "person" => {
                m.insert(index, Box::new(Person{name: "default".to_string()}));
            },
            "animal" => {
                m.insert(index, Box::new(Animal{name: "default".to_string()}));
            },
            _ => {

            }
        }
    });

    for (index, _) in items.iter().enumerate() {
        println!("{:?}", m.get(&index).unwrap().greet());
    }
}