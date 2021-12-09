use std::collections::HashMap;
use std::collections::HashSet;
use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day9_part1(values)),
        "2" => println!("{}", day9_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn find_low_points(map: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32, i32)> {
    let mut low_points: Vec<(i32, i32, i32)> = vec![];

    for (pos, height) in map.iter() {
        let west = map.get(&(pos.0 - 1, pos.1));
        let east = map.get(&(pos.0 + 1, pos.1));
        let north = map.get(&(pos.0, pos.1 - 1));
        let south = map.get(&(pos.0, pos.1 + 1));

        if west.is_some() && west.unwrap() < height {
            continue;
        }
        if east.is_some() && east.unwrap() < height {
            continue;
        }
        if north.is_some() && north.unwrap() < height {
            continue;
        }
        if south.is_some() && south.unwrap() < height {
            continue;
        }
        low_points.push((pos.0, pos.1, *height));
    }

    low_points
}

fn day9_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut map = HashMap::new();

    for (row, line) in v.enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let height = cell.to_digit(10).unwrap() as i32;
            map.insert((row as i32, col as i32), height);
        }
    }

    let low_points = find_low_points(&map);

    low_points.iter().fold(0, |acc, x| acc + x.2 + 1)
}

#[test]
fn day9_part1_test() {
    let v = vec![
        String::from("2199943210"),
        String::from("3987894921"),
        String::from("9856789892"),
        String::from("8767896789"),
        String::from("9899965678"),
    ];
    let answer = day9_part1(v.into_iter());

    assert_eq!(15, answer);
}

fn walk_basin(
    mut basin: &mut HashSet<(i32, i32)>,
    map: &HashMap<(i32, i32), i32>,
    pos: (i32, i32),
) {
    basin.insert(pos);

    let west = (pos.0, pos.1 - 1);
    let east = (pos.0, pos.1 + 1);
    let north = (pos.0 - 1, pos.1);
    let south = (pos.0 + 1, pos.1);

    let dirs = [west, east, north, south];

    let valid_dirs = dirs.iter().filter(|pos| match map.get(pos) {
        Some(height) => *height < 9,
        _ => false,
    });

    for dir in valid_dirs {
        if !basin.contains(dir) {
            walk_basin(&mut basin, &map, *dir);
        }
    }
}

fn day9_part2(v: impl Iterator<Item = String>) -> i32 {
    let mut map = HashMap::new();

    for (row, line) in v.enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let height = cell.to_digit(10).unwrap() as i32;
            map.insert((row as i32, col as i32), height);
        }
    }

    let low_points = find_low_points(&map);

    let basins = low_points.iter().map(|(i, j, _)| {
        let mut basin = HashSet::new();
        walk_basin(&mut basin, &map, (*i, *j));
        basin
    });

    basins
        .map(|x| x.len() as i32)
        .rev()
        .take(3)
        .fold(1, |acc, x| acc * x)
}

#[test]
fn day9_part2_test() {
    let v = vec![
        String::from("2199943210"),
        String::from("3987894921"),
        String::from("9856789892"),
        String::from("8767896789"),
        String::from("9899965678"),
    ];
    let answer = day9_part2(v.into_iter());

    assert_eq!(1134, answer);
}
