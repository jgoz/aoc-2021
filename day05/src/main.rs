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

type Coords = HashMap<(i32, i32), i32>;

fn mark_coords(coords: &mut Coords, start: (i32, i32), end: (i32, i32)) {
    let (x1, y1) = start;
    let (x2, y2) = end;
    let x_off = if x2 > x1 { 1 } else { -1 };
    let y_off = if y2 > y1 { 1 } else { -1 };
    let mut x = x1;
    let mut y = y1;
    loop {
        let coord = (x, y);
        if let Some(val) = coords.get_mut(&coord) {
            *val += 1
        } else {
            coords.insert(coord, 1);
        }

        if x == x2 && y == y2 {
            break;
        }
        if x != x2 {
            x += x_off
        }
        if y != y2 {
            y += y_off
        }
    }
}

fn day5_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut map: Coords = HashMap::<(i32, i32), i32>::new();

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

        let start = (start_parts[0], start_parts[1]);
        let end = (end_parts[0], end_parts[1]);

        if start.0 == end.0 || start.1 == end.1 {
            mark_coords(&mut map, start, end);
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
    let mut map: Coords = HashMap::<(i32, i32), i32>::new();

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

        let start = (start_parts[0], start_parts[1]);
        let end = (end_parts[0], end_parts[1]);

        mark_coords(&mut map, start, end);
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
