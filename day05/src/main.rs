use std::collections::HashMap;
use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day5_part1(values)),
        "2" => println!("{}", day5_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl IntoIterator for Line {
    type Item = (i32, i32);
    type IntoIter = LineIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let x_step = (self.x2 - self.x1).signum();
        let y_step = (self.y2 - self.y1).signum();

        LineIntoIter {
            x: self.x1 - x_step,
            y: self.y1 - y_step,
            x_max: self.x2,
            y_max: self.y2,
            x_step,
            y_step,
        }
    }
}

struct LineIntoIter {
    x: i32,
    y: i32,
    x_max: i32,
    y_max: i32,
    x_step: i32,
    y_step: i32,
}

impl Iterator for LineIntoIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let x_finished = self.x == self.x_max;
        let y_finished = self.y == self.y_max;

        if x_finished && y_finished {
            return None;
        }
        if x_finished {
            self.x_step = 0;
        }
        if y_finished {
            self.y_step = 0;
        }

        self.x += self.x_step;
        self.y += self.y_step;
        Some((self.x, self.y))
    }
}

fn day5_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut map = HashMap::<(i32, i32), i32>::new();

    for line in v {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("Bad input! {}", line)
        }
        let start_parts = parts[0]
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let end_parts = parts[1]
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if start_parts.len() != 2 || end_parts.len() != 2 {
            panic!("Bad input! {}", line)
        }

        let line = Line {
            x1: start_parts[0],
            x2: end_parts[0],
            y1: start_parts[1],
            y2: end_parts[1],
        };

        if line.x1 == line.x2 || line.y1 == line.y2 {
            for coord in line {
                if let Some(val) = map.get_mut(&coord) {
                    *val += 1
                } else {
                    map.insert(coord, 1);
                }
            }
        }
    }

    map.iter().fold(
        0,
        |sum, (_coord, count)| if *count >= 2 { sum + 1 } else { sum },
    )
}

#[test]
fn day5_part1_test() {
    let v = vec![
        String::from("0,9 -> 5,9"),
        String::from("8,0 -> 0,8"),
        String::from("9,4 -> 3,4"),
        String::from("2,2 -> 2,1"),
        String::from("7,0 -> 7,4"),
        String::from("6,4 -> 2,0"),
        String::from("0,9 -> 2,9"),
        String::from("3,4 -> 1,4"),
        String::from("0,0 -> 8,8"),
        String::from("5,5 -> 8,2"),
    ];
    let answer = day5_part1(v.into_iter());

    assert_eq!(5, answer);
}

fn day5_part2(v: impl Iterator<Item = String>) -> i32 {
    let mut map = HashMap::<(i32, i32), i32>::new();

    for line in v {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("Bad input! {}", line)
        }
        let start_parts = parts[0]
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let end_parts = parts[1]
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if start_parts.len() != 2 || end_parts.len() != 2 {
            panic!("Bad input! {}", line)
        }

        let line = Line {
            x1: start_parts[0],
            x2: end_parts[0],
            y1: start_parts[1],
            y2: end_parts[1],
        };

        for coord in line {
            if let Some(val) = map.get_mut(&coord) {
                *val += 1
            } else {
                map.insert(coord, 1);
            }
        }
    }

    map.iter().fold(
        0,
        |sum, (_coord, count)| if *count >= 2 { sum + 1 } else { sum },
    )
}

#[test]
fn day5_part2_test() {
    let v = vec![
        String::from("0,9 -> 5,9"),
        String::from("8,0 -> 0,8"),
        String::from("9,4 -> 3,4"),
        String::from("2,2 -> 2,1"),
        String::from("7,0 -> 7,4"),
        String::from("6,4 -> 2,0"),
        String::from("0,9 -> 2,9"),
        String::from("3,4 -> 1,4"),
        String::from("0,0 -> 8,8"),
        String::from("5,5 -> 8,2"),
    ];
    let answer = day5_part2(v.into_iter());

    assert_eq!(12, answer);
}
