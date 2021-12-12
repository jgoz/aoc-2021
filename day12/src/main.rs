use std::{
    collections::{HashMap, HashSet},
    env, io,
    io::prelude::*,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day12_part1(values)),
        "2" => println!("{}", day12_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone)]
struct Arena {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    id: NodeId,
    big: bool,
    start: bool,
    end: bool,
    siblings: Vec<NodeId>,
}

impl Node {
    fn new(name: &String, id: &NodeId) -> Node {
        let end = name == "end";
        let start = name == "start";
        Node {
            name: name.clone(),
            id: id.clone(),
            big: name.to_uppercase() == *name,
            start,
            end,
            siblings: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct NodeId {
    index: usize,
}

type Path = Vec<NodeId>;

struct Cave {
    arena: Arena,
    start: Option<NodeId>,
}

impl Cave {
    fn from(v: impl Iterator<Item = String>) -> Cave {
        let mut arena = Arena { nodes: vec![] };
        let mut nodes: HashMap<String, NodeId> = HashMap::new();
        let mut next_index = 0;

        for line in v {
            let names = line.split("-").collect::<Vec<_>>();

            let name_one = names.get(0).unwrap().to_string();
            let name_two = names.get(1).unwrap().to_string();

            if !nodes.contains_key(&name_one) {
                let id = NodeId { index: next_index };
                next_index += 1;

                arena.nodes.push(Node::new(&name_one, &id));
                nodes.insert(name_one.clone(), id);
            }

            if !nodes.contains_key(&name_two) {
                let id = NodeId { index: next_index };
                next_index += 1;

                arena.nodes.push(Node::new(&name_two, &id));
                nodes.insert(name_two.clone(), id);
            }

            arena.nodes[nodes.get(&name_one).unwrap().index]
                .siblings
                .push(nodes.get(&name_two).unwrap().clone());

            arena.nodes[nodes.get(&name_two).unwrap().index]
                .siblings
                .push(nodes.get(&name_one).unwrap().clone());
        }

        Cave {
            arena,
            start: Some(nodes.get("start").unwrap().clone()),
        }
    }

    fn walk(&self, max_small_visits: usize) -> HashSet<Path> {
        let mut paths: HashSet<Path> = HashSet::new();
        let start_id = self.start.as_ref().unwrap();

        let mut queue: Vec<(NodeId, Path)> = Vec::new();
        queue.push((start_id.clone(), vec![*start_id]));

        while let Some((node_id, path)) = queue.pop() {
            let node = &self.arena.nodes[node_id.index];

            if node.end {
                paths.insert(path.clone());
                continue;
            }

            'siblings: for sibling_id in &node.siblings {
                let sibling = &self.arena.nodes[sibling_id.index];
                if sibling.start {
                    continue;
                }
                if sibling.end && path.contains(&sibling_id) {
                    continue;
                }

                if !sibling.big && path.contains(&sibling_id) {
                    for id in path.iter() {
                        let node = &self.arena.nodes[id.index];
                        if node.big || node.start || node.end {
                            continue;
                        }
                        if path.iter().filter(|x| *x == id).count() >= max_small_visits {
                            continue 'siblings;
                        }
                    }
                }

                let mut new_path = path.clone();
                new_path.push(sibling_id.clone());
                queue.push((sibling_id.clone(), new_path));
            }
        }

        paths
    }
}

fn day12_part1(v: impl Iterator<Item = String>) -> i32 {
    let cave = Cave::from(v);
    let paths = cave.walk(1);

    paths.len() as i32
}

#[test]
fn day12_part1_test_1() {
    let v = vec![
        String::from("start-A"),
        String::from("start-b"),
        String::from("A-c"),
        String::from("A-b"),
        String::from("b-d"),
        String::from("A-end"),
        String::from("b-end"),
    ];
    let answer = day12_part1(v.into_iter());

    assert_eq!(10, answer);
}

#[test]
fn day12_part1_test_2() {
    let v = vec![
        String::from("dc-end"),
        String::from("HN-start"),
        String::from("start-kj"),
        String::from("dc-start"),
        String::from("dc-HN"),
        String::from("LN-dc"),
        String::from("HN-end"),
        String::from("kj-sa"),
        String::from("kj-HN"),
        String::from("kj-dc"),
    ];
    let answer = day12_part1(v.into_iter());

    assert_eq!(19, answer);
}

fn day12_part2(v: impl Iterator<Item = String>) -> i32 {
    let cave = Cave::from(v);
    let paths = cave.walk(2);

    paths.len() as i32
}

#[test]
fn day12_part2_test_1() {
    let v = vec![
        String::from("start-A"),
        String::from("start-b"),
        String::from("A-c"),
        String::from("A-b"),
        String::from("b-d"),
        String::from("A-end"),
        String::from("b-end"),
    ];
    let answer = day12_part2(v.into_iter());

    assert_eq!(36, answer);
}

#[test]
fn day12_part2_test_2() {
    let v = vec![
        String::from("dc-end"),
        String::from("HN-start"),
        String::from("start-kj"),
        String::from("dc-start"),
        String::from("dc-HN"),
        String::from("LN-dc"),
        String::from("HN-end"),
        String::from("kj-sa"),
        String::from("kj-HN"),
        String::from("kj-dc"),
    ];
    let answer = day12_part2(v.into_iter());

    assert_eq!(103, answer);
}
