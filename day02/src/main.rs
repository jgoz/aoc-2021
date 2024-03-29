use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());
    match part.as_str() {
        "1" => println!("{}", part1_depth_by_horizontal(values)),
        "2" => println!("{}", part2_depth_by_horizontal(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn part1_depth_by_horizontal(v: impl Iterator<Item = String>) -> i32 {
    let mut depth = 0;
    let mut horiz = 0;
    for elem in v {
        let cap = elem.split(" ").collect::<Vec<_>>();
        let dir = cap.get(0).unwrap();
        let amt = cap.get(1).unwrap().parse::<i32>().unwrap();
        match *dir {
            "forward" => horiz += amt,
            "down" => depth += amt,
            "up" => depth -= amt,
            _ => println!("Invalid direction {}", dir),
        }
    }
    depth * horiz
}

#[test]
fn day2_part1_test() {
    let v = vec![
        String::from("forward 5"),
        String::from("down 5"),
        String::from("forward 8"),
        String::from("up 3"),
        String::from("down 8"),
        String::from("forward 2"),
    ];
    let answer = part1_depth_by_horizontal(v.into_iter());

    assert_eq!(150, answer);
}

fn part2_depth_by_horizontal(v: impl Iterator<Item = String>) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horiz = 0;
    for elem in v {
        let cap = elem.split(" ").collect::<Vec<_>>();
        let dir = cap.get(0).unwrap();
        let amt = cap.get(1).unwrap().parse::<i32>().unwrap();
        match *dir {
            "forward" => {
                horiz += amt;
                depth += aim * amt;
            }
            "down" => aim += amt,
            "up" => aim -= amt,
            _ => println!("Invalid direction {}", dir),
        }
    }
    depth * horiz
}

#[test]
fn day2_part2_test() {
    let v = vec![
        String::from("forward 5"),
        String::from("down 5"),
        String::from("forward 8"),
        String::from("up 3"),
        String::from("down 8"),
        String::from("forward 2"),
    ];
    let answer = part2_depth_by_horizontal(v.into_iter());

    assert_eq!(900, answer);
}
