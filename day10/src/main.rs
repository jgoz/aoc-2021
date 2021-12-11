use std::{collections::HashMap, env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day10_part1(values)),
        "2" => println!("{}", day10_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn day10_part1(v: impl Iterator<Item = String>) -> i32 {
    let start_pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let end_pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut sum = 0;

    'outer: for line in v {
        let mut stack = vec![];

        for c in line.chars() {
            if start_pairs.contains_key(&c) {
                stack.push(c);
            } else if end_pairs.contains_key(&c) {
                if stack.pop().unwrap() != *end_pairs.get(&c).unwrap() {
                    sum += points.get(&c).unwrap();
                    continue 'outer;
                }
            }
        }
    }

    sum
}

#[test]
fn day10_part1_test() {
    let v = vec![
        String::from("[({(<(())[]>[[{[]{<()<>>"),
        String::from("[(()[<>])]({[<{<<[]>>("),
        String::from("{([(<{}[<>[]}>{[]{[(<()>"),
        String::from("(((({<>}<{<{<>}{[]{[]{}"),
        String::from("[[<[([]))<([[{}[[()]]]"),
        String::from("[{[{({}]{}}([{[{{{}}([]"),
        String::from("{<[[]]>}<{[{[{[]{()[[[]"),
        String::from("[<(<(<(<{}))><([]([]()"),
        String::from("<{([([[(<>()){}]>(<<{{"),
        String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
    ];
    let answer = day10_part1(v.into_iter());

    assert_eq!(26397, answer);
}

fn day10_part2(v: impl Iterator<Item = String>) -> i64 {
    let start_pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let end_pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut sums = vec![];

    'outer: for line in v {
        let mut stack = vec![];

        for c in line.chars() {
            if start_pairs.contains_key(&c) {
                stack.push(c);
            } else if end_pairs.contains_key(&c) {
                let last = stack.pop().unwrap();
                if last != *end_pairs.get(&c).unwrap() {
                    continue 'outer;
                }
            }
        }

        let mut sum: i64 = 0;
        for c in stack.iter().rev() {
            sum *= 5;
            sum += points.get(start_pairs.get(c).unwrap()).unwrap();
        }
        sums.push(sum);
    }

    sums.sort();
    *sums.get(sums.len() / 2).unwrap()
}

#[test]
fn day10_part2_test() {
    let v = vec![
        String::from("[({(<(())[]>[[{[]{<()<>>"),
        String::from("[(()[<>])]({[<{<<[]>>("),
        String::from("{([(<{}[<>[]}>{[]{[(<()>"),
        String::from("(((({<>}<{<{<>}{[]{[]{}"),
        String::from("[[<[([]))<([[{}[[()]]]"),
        String::from("[{[{({}]{}}([{[{{{}}([]"),
        String::from("{<[[]]>}<{[{[{[]{()[[[]"),
        String::from("[<(<(<(<{}))><([]([]()"),
        String::from("<{([([[(<>()){}]>(<<{{"),
        String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
    ];
    let answer = day10_part2(v.into_iter());

    assert_eq!(288957, answer);
}
