use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
};

use petgraph::{
    dot::Dot,
    visit::{DfsPostOrder, Topo},
    Graph,
};

#[derive(Debug)]
struct Node {
    id: String,
    data: Data,
}

#[derive(Debug)]
enum Data {
    Value(u16),
    Add,
    Sub,
    Mul,
    Div,
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
        if let Ok(v) = value.parse::<u16>() {
            let node = Node {
                id: key.into(),
                data: Data::Value(v),
            };
            let i = graph.add_node(node);
            node_indices.insert(key, i);
        } else {
            let mut iter = value.split(' ');
            let (_oper_1, expr, _oper_2) = (
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
        }
    }

    for line in string.lines() {
        let mut iter = line.split(": ");
        let (key, value) = (iter.next().unwrap(), iter.next().unwrap());
        if let Err(_) = value.parse::<u16>() {
            let mut iter = value.split(' ');
            let (oper_1, _expr, oper_2) = (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            );

            let i = node_indices[key];
            let oper_1_index = node_indices[oper_1];
            let oper_2_index = node_indices[oper_2];
            graph.add_edge(i, oper_1_index, ());
            graph.add_edge(i, oper_2_index, ());
        }
    }

    let output = format!("{:?}", Dot::new(&graph));
    let mut file = File::create("/tmp/graph.dot")?;
    file.write_all(output.as_bytes())?;

    let root_index = node_indices["root"];
    let mut visitor = DfsPostOrder::new(&graph, root_index);

    let mut oper_1 = None;
    let mut oper_2 = None;

    let mut result = 0;
    while let Some(i) = visitor.next(&graph) {
        let n = graph.node_weight(i).unwrap();
        println!("{n:?}");

        match n.data {
            Data::Value(v) => {
                if oper_1.is_none() {
                    oper_1 = Some(v);
                } else if oper_2.is_none() {
                    oper_2 = Some(v);
                }
            }
            Data::Add => {
                result = oper_1.unwrap() + oper_2.unwrap();
                oper_1 = Some(result);
                oper_2 = None;
                *(graph.node_weight_mut(i).unwrap()) = Node {
                    id: n.id.clone(),
                    data: Data::Value(result),
                };
                let n = graph.node_weight(i).unwrap();
                println!("After Add {n:?}");
            }
            Data::Sub => {
                result = oper_1.unwrap() - oper_2.unwrap();
                oper_1 = Some(result);
                oper_2 = None;
                *(graph.node_weight_mut(i).unwrap()) = Node {
                    id: n.id.clone(),
                    data: Data::Value(result),
                };
                let n = graph.node_weight(i).unwrap();
                println!("After Sub {n:?}");
            }
            Data::Mul => {
                result = oper_1.unwrap() * oper_2.unwrap();
                oper_1 = Some(result);
                oper_2 = None;
                *(graph.node_weight_mut(i).unwrap()) = Node {
                    id: n.id.clone(),
                    data: Data::Value(result),
                };
                let n = graph.node_weight(i).unwrap();
                println!("After Mul {n:?}");
            }
            Data::Div => {
                result = oper_1.unwrap() / oper_2.unwrap();
                oper_1 = Some(result);
                oper_2 = None;
                *(graph.node_weight_mut(i).unwrap()) = Node {
                    id: n.id.clone(),
                    data: Data::Value(result),
                };
                let n = graph.node_weight(i).unwrap();
                println!("After Div {n:?}");
            }
        }
    }
    println!("{oper_1:?} {oper_2:?}");

    Ok(())
}
