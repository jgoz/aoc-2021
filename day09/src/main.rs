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

fn find_low_points(map: &Vec<Vec<i32>>) -> Vec<(i32, i32, i32)> {
    let mut low_points: Vec<(i32, i32, i32)> = vec![];

    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if j > 0 && cell >= &row[j - 1] {
                continue;
            }
            if j < row.len() - 1 && cell >= &row[j + 1] {
                continue;
            }
            if i > 0 && cell >= &map[i - 1][j] {
                continue;
            }
            if i < map.len() - 1 && cell >= &map[i + 1][j] {
                continue;
            }
            low_points.push((i as i32, j as i32, *cell));
        }
    }

    low_points
}

fn day9_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut map = Vec::new();

    for line in v {
        let heights = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        map.push(heights)
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

fn walk_basin(mut basin: &mut HashSet<(i32, i32)>, map: &Vec<Vec<i32>>, i: usize, j: usize) {
    basin.insert((i as i32, j as i32));

    // Look east
    if j > 0 && map[i][j - 1] < 9 && !basin.contains(&(i as i32, (j as i32) - 1)) {
        walk_basin(&mut basin, &map, i, j - 1);
    }

    // Look west
    if j < map[i].len() - 1 && map[i][j + 1] < 9 && !basin.contains(&(i as i32, (j as i32) + 1)) {
        walk_basin(&mut basin, &map, i, j + 1);
    }

    // Look north
    if i > 0 && map[i - 1][j] < 9 && !basin.contains(&((i as i32) - 1, j as i32)) {
        walk_basin(&mut basin, &map, i - 1, j);
    }

    // Look south
    if i < map.len() - 1 && map[i + 1][j] < 9 && !basin.contains(&((i as i32) + 1, j as i32)) {
        walk_basin(&mut basin, &map, i + 1, j);
    }
}

fn day9_part2(v: impl Iterator<Item = String>) -> i32 {
    let mut map = Vec::new();

    for line in v {
        let heights = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        map.push(heights)
    }

    let low_points = find_low_points(&map);

    let mut basins = vec![];

    for (i, j, _) in low_points {
        let mut basin: HashSet<(i32, i32)> = HashSet::new();
        walk_basin(&mut basin, &map, i as usize, j as usize);
        basins.push(basin);
    }

    basins
        .iter()
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
