use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

use petgraph::Graph;

struct Node {
    id: String,
    data: Data,
}

enum Data {
    Value(u8),
    Add,
    Sub,
    Mul,
    Div,
    Uninit,
}

fn main() -> Result<(), io::Error> {
    let mut input = File::open("input.txt")?;

    let mut string = String::new();

    input.read_to_string(&mut string)?;

    let mut graph = Graph::<Node, ()>::new();
    let mut node_indices: HashMap<&str, _> = HashMap::new();

    for line in string.lines() {
        let mut iter = line.split(": ");
        let (key, value) = (iter.next().unwrap(), iter.next().unwrap());
        if let Ok(v) = value.parse::<u8>() {
            let node = Node {
                id: key.into(),
                data: Data::Value(v),
            };
            let i = graph.add_node(node);
            node_indices.insert(key, i);
        } else {
            let mut iter = value.split(' ');
            let (oper_1, expr, oper_2) = (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            );

            let data = match expr {
                "+" => Data::Add,
                "-" => Data::Sub,
                "*" => Data::Mul,
                "/" => Data::Div,
                _ => unreachable!("Incorrect expression"),
            };

            let i = graph.add_node(Node {
                id: key.into(),
                data,
            });
            node_indices.insert(key, i);

            {
                let oper_1_index = node_indices.entry(oper_1).or_insert_with(|| {
                    graph.add_node(Node {
                        id: key.into(),
                        data: Data::Uninit,
                    })
                });
                graph.add_edge(i, *oper_1_index, ());
            }
            {
                let oper_2_index = node_indices.entry(oper_2).or_insert_with(|| {
                    graph.add_node(Node {
                        id: key.into(),
                        data: Data::Uninit,
                    })
                });
                graph.add_edge(i, *oper_2_index, ());
            }
        };
    }

    Ok(())
}
