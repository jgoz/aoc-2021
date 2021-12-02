use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap().parse::<i32>().unwrap());
    if part == "1" {
        println!("{}", larger_than_previous(values));
    } else if part == "2" {
        println!("{}", windowed_larger_than_previous(values));
    } else {
        println!("{}", "Invalid part");
    }
}

fn larger_than_previous(mut v: impl Iterator<Item = i32>) -> i32 {
    let mut prev = v.next();
    if prev.is_none() {
        return 0;
    }
    let mut num_larger = 0;
    for elem in v {
        if elem > prev.unwrap() {
            num_larger += 1;
        }
        prev = Some(elem);
    }
    num_larger
}

#[test]
fn part1_test() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let answer = larger_than_previous(v.into_iter());

    assert_eq!(7, answer);
}

fn windowed_larger_than_previous(v: impl Iterator<Item = i32>) -> i32 {
    let mut num_larger = 0;
    let mut running_sum = 0;
    let mut window: Vec<i32> = vec![];
    for elem in v {
        if window.len() < 3 {
            window.push(elem);
            running_sum += elem;
            continue;
        }

        let prev_sum = running_sum;

        running_sum -= window[0];
        window.remove(0);
        window.push(elem);
        running_sum += elem;

        if running_sum > prev_sum {
            num_larger += 1;
        }
    }

    num_larger
}

#[test]
fn part2_test() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let answer = windowed_larger_than_previous(v.into_iter());

    assert_eq!(5, answer);
}
