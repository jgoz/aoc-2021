use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day6(values, 80)),
        "2" => println!("{}", day6(values, 256)),
        _ => println!("Invalid part {}", part),
    }
}

fn day6(mut v: impl Iterator<Item = String>, days: i64) -> i64 {
    let line = v.next().unwrap();
    let initial = line.split(",").map(|x| x.parse::<usize>().unwrap());

    let mut fish = [0; 9];
    for f in initial {
        fish[f] += 1;
    }

    for _ in 0..days {
        let fish_0 = fish[0];
        for f in 0..8 {
            fish[f] = fish[f + 1];
        }
        fish[6] += fish_0;
        fish[8] = fish_0;
    }

    fish.into_iter().fold(0, |acc, v| acc + v)
}

#[test]
fn day6_part1_test() {
    let v = vec![String::from("3,4,3,1,2")];
    let answer = day6(v.into_iter(), 80);

    assert_eq!(5934, answer);
}

#[test]
fn day6_part2_test() {
    let v = vec![String::from("3,4,3,1,2")];
    let answer = day6(v.into_iter(), 256);

    assert_eq!(26984457539, answer);
}
