use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day7_part1(values)),
        "2" => println!("{}", day7_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn day7_part1(mut v: impl Iterator<Item = String>) -> i32 {
    let line = v.next().unwrap();
    let positions: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    let mut min = i32::max_value();

    for i in min_pos..=max_pos {
        let cost = positions.iter().fold(0, |sum, p| sum + (p - i).abs());

        if cost < min {
            min = cost;
        }
    }

    min
}

#[test]
fn day7_part1_test() {
    let v = vec![String::from("16,1,2,0,4,2,7,1,2,14")];
    let answer = day7_part1(v.into_iter());

    assert_eq!(37, answer);
}

fn day7_part2(mut v: impl Iterator<Item = String>) -> i32 {
    let line = v.next().unwrap();
    let positions: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    let mut min = i32::max_value();

    for i in min_pos..=max_pos {
        let cost = positions.iter().fold(0, |sum, p| {
            let distance = (p - i).abs();
            let sub_cost = distance * (distance + 1) / 2;
            sum + sub_cost
        });

        if cost < min {
            min = cost;
        }
    }

    min
}

#[test]
fn day7_part2_test() {
    let v = vec![String::from("16,1,2,0,4,2,7,1,2,14")];
    let answer = day7_part2(v.into_iter());

    assert_eq!(168, answer);
}
