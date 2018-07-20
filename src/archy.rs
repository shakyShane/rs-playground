use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub nodes: Vec<Node>
}

impl Node {
    pub fn new (label: impl Into<String>, nodes: Vec<Node>) -> Node {
        Node {
            label: label.into(),
            nodes
        }
    }
}

pub fn archy(input: &Node, prefix: &str) -> String {
    let lines = input.label.lines().collect::<Vec<&str>>();
    let node_len = input.nodes.len();
    let suf = if node_len > 0 { "│" } else { " " };
    let splitter = format!("\n{}{} ", prefix, suf);
    let joined = lines.join(splitter.as_str());

    let child = input.nodes.iter().enumerate().map(|(ix, node)| {

        let last = ix == (node_len - 1);
        let more = node.nodes.len() > 0;
        let prefix_ = format!("{}{} ", prefix, if last { " " } else  { "│" });
        let next_string = &archy(&node, prefix_.as_str());
        let next_string_indices = next_string.char_indices();

        let target_num = prefix.char_indices().count() + 2;

        let next_output = next_string_indices.skip(target_num)
            .map(|(i, char)| char.to_string())
            .collect::<Vec<String>>()
            .join("");

        vec![
            prefix,
            if last { "└" } else { "├" },
            "─",
            if more { "┬" } else { "─" },
            " ",
            next_output.as_str()
        ].join("")

    }).collect::<Vec<String>>();

    format!("{}{}\n{}", prefix, joined, child.join(""))
}

#[test]
fn test_archy() {
    let node = Node::new("one", vec![
        Node::new("two\nlines", vec![
            Node::new("three", vec![
                Node::new("three-1", vec![
                    Node::new("Just some
really really
long label", vec![]),
                    Node::new("three-1-2", vec![]),
                    Node::new("three-1-3", vec![]),
                ]),
                Node::new("three-2", vec![]),
            ]),
            Node::new("four", vec![
                Node::new("two", vec![
                    Node::new("three",vec![]),
                    Node::new("four",vec![])
                ])
            ])
        ])
    ]);

    println!("{}", archy(&node, ""));
//    assert_eq!("one
//└─┬ two
//  │ lines
//  ├─┬ three
//  │ ├─┬ three-1
//  │ │ ├── three-1-1
//  │ │ │   second
//  │ │ │   third
//  │ │ ├── three-1-2
//  │ │ └── three-1-3
//  │ └── three-2
//  └─┬ four
//    └─┬ two
//      ├── three
//      └── four
//", archy(&node, ""));
}