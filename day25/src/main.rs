use std::{collections::HashMap, env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day25_part1(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cucumber {
    East,
    South,
}

impl Cucumber {
    fn from(c: char) -> Option<Cucumber> {
        match c {
            '>' => Some(Cucumber::East),
            'v' => Some(Cucumber::South),
            _ => None,
        }
    }
}

type Pos = (i32, i32);

#[derive(Debug, Clone)]
struct Grid {
    positions: HashMap<Pos, Cucumber>,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn from(v: impl Iterator<Item = String>) -> Grid {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut positions = HashMap::new();
        for (y, line) in v.enumerate() {
            for (x, c) in line.char_indices() {
                let pos = (x as i32, y as i32);
                if let Some(cucumber) = Cucumber::from(c) {
                    positions.insert(pos, cucumber);
                }
            }

            max_x = line.len() as i32;
            max_y += 1;
        }

        Grid {
            positions,
            max_x: max_x - 1,
            max_y: max_y - 1,
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let pos = (x, y);
                if let Some(cucumber) = self.positions.get(&pos) {
                    s.push(match cucumber {
                        Cucumber::East => '>',
                        Cucumber::South => 'v',
                    });
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }
}

fn next_state(grid: &Grid) -> Option<Grid> {
    let mut east = grid.clone();
    let mut moves = 0;

    for (&(x, y), &cucumber) in grid.positions.iter().filter(|(_, &c)| c == Cucumber::East) {
        let new_pos = if x == grid.max_x { (0, y) } else { (x + 1, y) };

        if grid.positions.contains_key(&new_pos) {
            continue;
        }

        east.positions.remove(&(x, y));
        east.positions.insert(new_pos, cucumber);
        moves += 1;
    }

    let mut south = east.clone();

    for (&(x, y), &cucumber) in east.positions.iter().filter(|(_, &c)| c == Cucumber::South) {
        let new_pos = if y == grid.max_y { (x, 0) } else { (x, y + 1) };

        if east.positions.contains_key(&new_pos) {
            continue;
        }

        south.positions.remove(&(x, y));
        south.positions.insert(new_pos, cucumber);
        moves += 1;
    }

    if moves > 0 {
        Some(south)
    } else {
        None
    }
}

fn day25_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut grid = Grid::from(v);
    let mut iterations = 1;

    println!("{}", grid.to_string());

    while let Some(next_grid) = next_state(&grid) {
        grid = next_grid;
        iterations += 1;
    }

    iterations
}

#[test]
fn day25_part1_test() {
    let v = vec![
        String::from("v...>>.vv>"),
        String::from(".vv>>.vv.."),
        String::from(">>.>v>...v"),
        String::from(">>v>>.>.v."),
        String::from("v>v.vv.v.."),
        String::from(">.>>..v..."),
        String::from(".vv..>.>v."),
        String::from("v.v..>>v.v"),
        String::from("....v..v.>"),
    ];
    let answer = day25_part1(v.into_iter());

    assert_eq!(58, answer);
}
