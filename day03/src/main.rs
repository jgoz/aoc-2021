use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());
    match part.as_str() {
        "1" => println!("{}", part1_gamma_epsilon(values)),
        "2" => println!("{}", part2_o2_co2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn part1_gamma_epsilon(mut v: impl Iterator<Item = String>) -> i32 {
    let first = v.next().unwrap();
    let len = first.len();

    let mut zeros: Vec<i32> = vec![0; len];
    let mut ones: Vec<i32> = vec![0; len];

    for elem in v.chain([first]) {
        for (i, c) in elem.chars().enumerate() {
            match c {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => panic!("Invalid character {}", c),
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..len {
        let x = len - i - 1;
        if ones[i] > zeros[i] {
            gamma |= 1 << x;
        } else {
            epsilon |= 1 << x;
        }
    }

    gamma * epsilon
}

#[test]
fn day3_part1_test() {
    let v = vec![
        String::from("00100"),
        String::from("11110"),
        String::from("10110"),
        String::from("10111"),
        String::from("10101"),
        String::from("01111"),
        String::from("00111"),
        String::from("11100"),
        String::from("10000"),
        String::from("11001"),
        String::from("00010"),
        String::from("01010"),
    ];
    let answer = part1_gamma_epsilon(v.into_iter());

    assert_eq!(198, answer);
}

struct Node {
    children: Option<[Box<Node>; 2]>,
    weight: i32,
    bit: i32,
}

fn part2_o2_co2(v: impl Iterator<Item = String>) -> i32 {
    let mut root = Box::new(Node {
        children: None,
        weight: 0,
        bit: 0,
    });
    let mut current: &mut Box<Node>;

    // Pass through the input and build a weighted tree of nodes
    for elem in v {
        current = &mut root;
        let len = elem.len() as i32;

        for (i, c) in elem.char_indices() {
            if current.children.is_none() {
                current.children = Some([
                    Box::new(Node {
                        children: None,
                        bit: 0,
                        weight: 0,
                    }),
                    Box::new(Node {
                        children: None,
                        bit: 1 << len - (i as i32) - 1,
                        weight: 0,
                    }),
                ]);
            }

            let children = current.children.as_mut().unwrap();
            current = match c {
                '0' => &mut children[0],
                '1' => &mut children[1],
                _ => panic!("Invalid character {}", c),
            };

            current.weight += 1;
        }
    }

    let mut o2 = 0;
    current = &mut root;
    loop {
        o2 |= current.bit;

        let children = current.children.as_mut().unwrap();
        let (next, alt) = if children[1].weight >= children[0].weight {
            (1, 0)
        } else {
            (0, 1)
        };

        if children[next].children.is_some() {
            // Not a leaf node, continue
            current = &mut children[next];
        } else if children[alt].children.is_some() {
            // 'next' was a leaf node, so continue down the 'alt' path
            current = &mut children[alt];
        } else {
            if children[next].weight > 0 {
                // Terminating condition - add bit if a value was represented on this path
                o2 |= children[next].bit;
            }
            break;
        }
    }

    let mut co2 = 0;
    current = &mut root;
    loop {
        co2 |= current.bit;

        let children = current.children.as_mut().unwrap();
        let (next, alt) = if children[0].weight <= children[1].weight {
            (0, 1)
        } else {
            (1, 0)
        };

        if children[next].children.is_some() {
            current = &mut children[next];
        } else if children[alt].children.is_some() {
            current = &mut children[alt];
        } else {
            if children[next].weight > 0 {
                co2 |= children[next].bit;
            }
            break;
        }
    }

    o2 * co2
}

fn _part2_o2_co2_naive(v: impl Iterator<Item = String>) -> i32 {
    let mut o2_candidates: Vec<String> = v.collect();
    let len = o2_candidates[0].len();

    let mut co2_candidates: Vec<String> = o2_candidates.clone();

    for i in 0..len {
        if o2_candidates.len() > 1 {
            let (o2_zeros, o2_ones): (Vec<String>, Vec<String>) = o2_candidates
                .clone()
                .into_iter()
                .partition(|s| s.chars().nth(i).unwrap() == '0');

            o2_candidates = if o2_ones.len() >= o2_zeros.len() {
                o2_ones
            } else {
                o2_zeros
            };
        }

        if co2_candidates.len() > 1 {
            let (co2_zeros, co2_ones): (Vec<String>, Vec<String>) = co2_candidates
                .clone()
                .into_iter()
                .partition(|s| s.chars().nth(i).unwrap() == '0');

            co2_candidates = if co2_zeros.len() <= co2_ones.len() {
                co2_zeros
            } else {
                co2_ones
            };
        }
    }

    let mut o2 = 0;
    for (i, c) in o2_candidates[0].chars().enumerate() {
        o2 |= match c {
            '1' => 1 << (len - i - 1),
            _ => 0,
        }
    }

    let mut co2 = 0;
    for (i, c) in co2_candidates[0].chars().enumerate() {
        co2 |= match c {
            '1' => 1 << (len - i - 1),
            _ => 0,
        }
    }

    o2 * co2
}

#[test]
fn day3_part2_test() {
    let v = vec![
        String::from("00100"),
        String::from("11110"),
        String::from("10110"),
        String::from("10111"),
        String::from("10101"),
        String::from("01111"),
        String::from("00111"),
        String::from("11100"),
        String::from("10000"),
        String::from("11001"),
        String::from("00010"),
        String::from("01010"),
    ];
    let answer = part2_o2_co2(v.into_iter());

    assert_eq!(230, answer);
}

#[test]
fn day3_part2_naive_test() {
    let v = vec![
        String::from("00100"),
        String::from("11110"),
        String::from("10110"),
        String::from("10111"),
        String::from("10101"),
        String::from("01111"),
        String::from("00111"),
        String::from("11100"),
        String::from("10000"),
        String::from("11001"),
        String::from("00010"),
        String::from("01010"),
    ];
    let answer = _part2_o2_co2_naive(v.into_iter());

    assert_eq!(230, answer);
}
