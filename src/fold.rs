#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub children: Vec<Item>
}

///
/// The purpose of this demo is to show how to thread a
/// `&mut Vec` through a fold on a separate collection
///
/// Note how the lifetime is needed on the first param 'items'
/// and on the return value. This encodes the fact that 'items' is
/// not being cloned at any point, but is returned at the end of each fold
///
pub fn collect<'a>(items: &'a mut Vec<Node>, children: &Vec<Item>) -> &'a mut Vec<Node> {
    children.iter().fold(items, |acc, item| {
        acc.push(Node{
            name: item.name.to_string()
        });
        collect(acc, &item.children)
    })
}

pub fn main() {

    // a dummy input, just something that has nested children
    let children = vec![
        Item {
            name: "first".into(),
            children: vec![
                Item {
                    name: "second".into(),
                    children: vec![
                        Item {
                            name: "third".into(),
                            children: vec![

                            ],
                        }
                    ],
                }
            ],
        }
    ];

    // a vec to hold a flattened list of nodes
    let mut input = vec![];

    // process the true recursively
    let result = collect(&mut input, &children);

    assert_eq!(result, &mut vec![
        Node { name: "first".to_string() },
        Node { name: "second".to_string() },
        Node { name: "third".to_string() }
    ]);
}
