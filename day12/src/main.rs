use std::{collections::HashMap, env, io, io::prelude::*};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeId {
    name: String,
}

impl NodeId {
    fn from(name: &str) -> NodeId {
        NodeId {
            name: name.to_string(),
        }
    }
}

type Path = Vec<NodeId>;

#[derive(Debug, Clone, PartialEq)]
enum NodeKind {
    Start,
    End,
    Big,
    Small,
}

#[derive(Debug, Clone)]
struct Node {
    id: NodeId,
    kind: NodeKind,
    siblings: Vec<NodeId>,
}

impl Node {
    fn new(id: &NodeId, siblings: Vec<NodeId>) -> Node {
        let kind = match id.name.as_str() {
            "start" => NodeKind::Start,
            "end" => NodeKind::End,
            n => {
                if id.name.to_uppercase() == n {
                    NodeKind::Big
                } else {
                    NodeKind::Small
                }
            }
        };
        Node {
            id: id.clone(),
            kind,
            siblings,
        }
    }

    fn is_start(&self) -> bool {
        self.kind == NodeKind::Start
    }
    fn is_end(&self) -> bool {
        self.kind == NodeKind::End
    }
    fn is_small(&self) -> bool {
        self.kind == NodeKind::Small
    }
}

struct Cave {
    nodes: HashMap<NodeId, Node>,
}

impl Cave {
    fn from(v: impl Iterator<Item = String>) -> Cave {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();

        for line in v {
            let (name_one, name_two) = line.split_once("-").unwrap();
            let id_one = NodeId::from(name_one);
            let id_two = NodeId::from(name_two);

            if !nodes.contains_key(&id_one) {
                nodes.insert(id_one.clone(), Node::new(&id_one, vec![id_two.clone()]));
            } else {
                let node_one = nodes.get_mut(&id_one).unwrap();
                node_one.siblings.push(id_two.clone());
            }
            if !nodes.contains_key(&id_two) {
                nodes.insert(id_two.clone(), Node::new(&id_two, vec![id_one]));
            } else {
                let node_two = nodes.get_mut(&id_two).unwrap();
                node_two.siblings.push(id_one.clone());
            }
        }

        Cave { nodes }
    }

    fn count_paths(&self, max_small_visits: usize) -> i32 {
        let mut paths = 0;
        let mut queue: Vec<(NodeId, Path)> = Vec::new();

        let start_id = NodeId::from("start");
        queue.push((start_id.clone(), vec![start_id]));

        let smalls = self.nodes.values().filter(|n| n.is_small());

        while let Some((node_id, path)) = queue.pop() {
            let node = &self.nodes[&node_id];

            if node.kind == NodeKind::End {
                paths += 1;
                continue;
            }

            for sibling_id in &node.siblings {
                let sibling = &self.nodes[&sibling_id];
                let exists_in_path = path.contains(&sibling_id);

                if sibling.is_start() || sibling.is_end() && exists_in_path {
                    continue;
                }
                if sibling.is_small() && exists_in_path {
                    let exceeded = smalls
                        .clone()
                        .any(|n| path.iter().filter(|id| **id == n.id).count() >= max_small_visits);
                    if exceeded {
                        continue;
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
    let paths = cave.count_paths(1);

    paths
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
    let paths = cave.count_paths(2);

    paths
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
