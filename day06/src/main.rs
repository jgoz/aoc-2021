use std::collections::HashMap;
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

fn day6(v: impl Iterator<Item = String>, days: i64) -> i64 {
    let initial = v
        .collect::<Vec<_>>()
        .get(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut fish = HashMap::from([
        (0, 0),
        (1, initial.iter().filter(|x| **x == 1).count() as i64),
        (2, initial.iter().filter(|x| **x == 2).count() as i64),
        (3, initial.iter().filter(|x| **x == 3).count() as i64),
        (4, initial.iter().filter(|x| **x == 4).count() as i64),
        (5, initial.iter().filter(|x| **x == 5).count() as i64),
        (6, initial.iter().filter(|x| **x == 6).count() as i64),
        (7, initial.iter().filter(|x| **x == 7).count() as i64),
        (8, 0),
    ]);

    for _ in 0..days {
        let fish_0 = fish.insert(0, fish[&1]).unwrap_or(0);
        fish.insert(1, fish[&2]);
        fish.insert(2, fish[&3]);
        fish.insert(3, fish[&4]);
        fish.insert(4, fish[&5]);
        fish.insert(5, fish[&6]);
        fish.insert(6, fish[&7] + fish_0);
        fish.insert(7, fish[&8]);
        fish.insert(8, fish_0);
    }

    fish.into_iter().fold(0, |acc, (_, v)| acc + v)
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
