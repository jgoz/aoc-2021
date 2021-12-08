use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", dayD_part1(values)),
        "2" => println!("{}", dayD_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn dayD_part1(mut v: impl Iterator<Item = String>) -> i32 {
    0
}

#[test]
fn dayD_part1_test() {
    let v = vec![
        String::from("")
    ];
    let answer = dayD_part1(v.into_iter());

    assert_eq!(0, answer);
}

fn dayD_part2(mut v: impl Iterator<Item = String>) -> i32 {
    0
}

#[test]
fn dayD_part2_test() {
    let v = vec![
        String::from("")
    ];
    let answer = dayD_part2(v.into_iter());

    assert_eq!(0, answer);
}
