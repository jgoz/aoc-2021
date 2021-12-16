use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    env, io,
    io::prelude::*,
    isize,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day15_part1(values)),
        "2" => println!("{}", day15_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

const MAX_RISK: isize = 10_000_000;
const DX: [isize; 4] = [-1, 0, 1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

/// (x, y)
type Coord = (isize, isize);

struct Cavern {
    map: HashMap<Coord, isize>,
    max_x: isize,
    max_y: isize,
    distances: HashMap<Coord, isize>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: Coord,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.0.cmp(&self.position.0))
            .then_with(|| other.position.1.cmp(&self.position.1))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn wrap(c: char, dx: isize, dy: isize) -> isize {
    let d = c.to_digit(10).unwrap() as isize;
    let next = d + dx + dy;
    if next > 9 {
        next - 9
    } else {
        next
    }
}

impl Cavern {
    pub fn from(v: impl Iterator<Item = String>) -> Cavern {
        let mut map = HashMap::new();
        let mut max_x: isize = 0;
        let mut max_y: isize = 0;
        for (y, line) in v.enumerate() {
            max_x = line.len() as isize;
            for (x, c) in line.chars().enumerate() {
                map.insert((x as isize, y as isize), c.to_digit(10).unwrap() as isize);
            }
            max_y += 1;
        }
        let mut distances = HashMap::new();
        for y in 0..max_y {
            for x in 0..max_x {
                distances.insert((x, y), MAX_RISK);
            }
        }
        distances.insert((0, 0), 0);
        Cavern {
            map,
            max_x: max_x - 1,
            max_y: max_y - 1,
            distances,
        }
    }

    pub fn from_2(v: impl Iterator<Item = String>) -> Cavern {
        let mut map = HashMap::new();
        let mut distances = HashMap::new();
        let lines = v.collect::<Vec<_>>();
        let max_x: isize = lines.get(0).unwrap().len() as isize;
        let max_y: isize = lines.len() as isize;
        for (y_u, line) in lines.iter().enumerate() {
            let y = y_u as isize;
            for dy in 0..5 {
                for (x_u, c) in line.chars().enumerate() {
                    let x = x_u as isize;
                    for dx in 0..5 {
                        map.insert((x + max_x * dx, y + max_y * dy), wrap(c, dx, dy));
                        distances.insert((x + max_x * dx, y + max_y * dy), MAX_RISK);
                    }
                }
            }
        }
        distances.insert((0, 0), 0);
        Cavern {
            map,
            max_x: max_x * 5 - 1,
            max_y: max_y * 5 - 1,
            distances,
        }
    }

    pub fn min_path(&mut self) -> isize {
        let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
            position: (0, 0),
            cost: 0,
        }]);

        while queue.len() > 0 {
            let State {
                position: head,
                cost: _,
            } = queue.pop().unwrap();

            let head_dist = *self.distances.get(&head).unwrap();

            for i in 0..4 {
                let x = head.0 + DX[i];
                let y = head.1 + DY[i];
                if x < 0 || y < 0 || x > self.max_x || y > self.max_y {
                    continue;
                }

                let cur = (x, y);
                let cur_dist = *self.distances.get(&cur).unwrap();
                let cost = *self.map.get(&cur).unwrap();

                if head_dist + cost < cur_dist {
                    // We found a shorter path to this cell, so update it and follow it next round
                    self.distances.insert(cur, head_dist + cost);
                    queue.push(State {
                        position: cur,
                        cost: cur_dist,
                    })
                }
            }
        }

        *self.distances.get(&(self.max_x, self.max_y)).unwrap()
    }
}

fn day15_part1(v: impl Iterator<Item = String>) -> isize {
    let mut cavern = Cavern::from(v);
    cavern.min_path()
}

#[test]
fn day15_part1_test() {
    let v = vec![
        String::from("1163751742"),
        String::from("1381373672"),
        String::from("2136511328"),
        String::from("3694931569"),
        String::from("7463417111"),
        String::from("1319128137"),
        String::from("1359912421"),
        String::from("3125421639"),
        String::from("1293138521"),
        String::from("2311944581"),
    ];
    let answer = day15_part1(v.into_iter());

    assert_eq!(40, answer);
}

fn day15_part2(v: impl Iterator<Item = String>) -> isize {
    let mut cavern = Cavern::from_2(v);
    cavern.min_path()
}

#[test]
fn day15_part2_test() {
    let v = vec![
        String::from("1163751742"),
        String::from("1381373672"),
        String::from("2136511328"),
        String::from("3694931569"),
        String::from("7463417111"),
        String::from("1319128137"),
        String::from("1359912421"),
        String::from("3125421639"),
        String::from("1293138521"),
        String::from("2311944581"),
    ];
    let answer = day15_part2(v.into_iter());

    assert_eq!(315, answer);
}
