use std::{collections::HashMap, env, io, io::prelude::*, isize};

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

struct Cavern {
    map: HashMap<(isize, isize), isize>,
    max_x: isize,
    max_y: isize,
    distances: HashMap<(isize, isize), isize>,
}

// x, y, distance
type Cell = (isize, isize, isize);

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
        let mut queue: Vec<Cell> = vec![(0, 0, 0)];
        while queue.len() > 0 {
            let head = *queue.get(0).unwrap();
            queue.remove(0);

            let head_dist = *self.distances.get(&(head.0, head.1)).unwrap();

            for i in 0..4 {
                let x = head.0 + DX[i];
                let y = head.1 + DY[i];
                if x < 0 || y < 0 || x > self.max_x || y > self.max_y {
                    continue;
                }

                let cur_dist = *self.distances.get(&(x, y)).unwrap();
                let cost = *self.map.get(&(x, y)).unwrap();

                if head_dist + cost < cur_dist {
                    // We found a shorter path to this cell, so update it and follow it next round
                    self.distances.insert((x, y), head_dist + cost);
                    queue.push((x, y, cur_dist))
                }
            }

            // Re-prioritize the queue by shortest distance
            queue.sort_by(|a, b| a.2.cmp(&b.2))
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
