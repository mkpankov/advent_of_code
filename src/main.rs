use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
};

use petgraph::{
    dot::Dot,
    visit::{DfsPostOrder, EdgeRef, IntoNodeIdentifiers, IntoNodeReferences, NodeFiltered, Topo},
    Direction, Graph,
};

#[derive(Debug)]
struct Node {
    id: String,
    data: Data,
    position: u8,
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
                position: 0,
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
                position: 0,
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
            *&mut graph[oper_1_index].position = 0;
            *&mut graph[oper_2_index].position = 1;
        }
    }

    let output = format!("{:?}", Dot::new(&graph));
    let mut file = File::create("/tmp/graph.dot")?;
    file.write_all(output.as_bytes())?;

    let nf = NodeFiltered::from_fn(&graph, |node_id| {
        // let node = graph.node_weight(node_id).unwrap();
        let outgoing_edges = graph.edges_directed(node_id, Direction::Outgoing);
        let mut len = 0;
        outgoing_edges
            .map(|e| graph.node_weight(e.target()).unwrap())
            .enumerate()
            .all(|(i, n)| {
                len = i;
                match n.data {
                    Data::Value(_) => true,
                    _ => false,
                }
            })
            && len > 0
    });
    let node_ids: Vec<_> = nf.node_identifiers().collect();
    for i in node_ids {
        let n = &graph[i];
        println!("{i:?} {n:?}");
        let outgoing_edges = graph.edges_directed(i, Direction::Outgoing);
        let mut child_nodes: Vec<_> = outgoing_edges
            .map(|e| (e.target(), graph.node_weight(e.target()).unwrap()))
            .collect();

        child_nodes.sort_by(|(_, a), (_, b)| a.position.cmp(&b.position));

        let child_values: Vec<_> = child_nodes
            .into_iter()
            .flat_map(|(_, n)| match n.data {
                Data::Value(v) => Some(v),
                _ => None,
            })
            .collect();

        println!("{} {}", child_values[0], child_values[1]);

        let n = &mut graph[i];

        *n = Node {
            id: n.id.clone(),
            data: Data::Value(match n.data {
                Data::Add => child_values[0] + child_values[1],
                Data::Sub => child_values[0] - child_values[1],
                Data::Mul => child_values[0] * child_values[1],
                Data::Div => child_values[0] / child_values[1],
                _ => unreachable!(),
            }),
            position: n.position,
        };
        println!("{i:?} {n:?}");
    }

    Ok(())
}
