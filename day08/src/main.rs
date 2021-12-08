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
        "1" => println!("{}", day8_part1(values)),
        "2" => println!("{}", day8_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let parts = line.split(" | ").into_iter().collect::<Vec<_>>();
    let input: Vec<String> = parts
        .get(0)
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let output: Vec<String> = parts
        .get(1)
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    (input, output)
}

fn day8_part1(v: impl Iterator<Item = String>) -> i32 {
    let length_to_segments: HashMap<usize, usize> = HashMap::from([(2, 1), (4, 4), (3, 7), (7, 8)]);

    let mut sum = 0;

    for line in v {
        let (_, output) = parse_line(&line);

        for val in output {
            if length_to_segments.contains_key(&val.len()) {
                sum += 1;
            }
        }
    }

    sum
}

#[test]
fn day8_part1_test() {
    let v = vec![
        String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
        String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
        String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
        String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
        String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
        String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
        String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
        String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
        String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
        String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
    ];
    let answer = day8_part1(v.into_iter());

    assert_eq!(26, answer);
}

fn day8_part2(v: impl Iterator<Item = String>) -> i32 {
    let length_to_segments: HashMap<usize, usize> = HashMap::from([(2, 1), (4, 4), (3, 7), (7, 8)]);

    let mut sum = 0;

    for line in v {
        let (input, output) = parse_line(&line);

        let (knowns, unknowns): (Vec<String>, Vec<String>) = input
            .into_iter()
            .partition(|x| length_to_segments.contains_key(&x.len()));

        let mut decoded: [String; 10] = [
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ];

        decoded[1] = knowns.iter().find(|x| x.len() == 2).unwrap().to_string();
        let one = HashSet::from_iter(decoded[1].chars());

        decoded[4] = knowns.iter().find(|x| x.len() == 4).unwrap().to_string();
        decoded[7] = knowns.iter().find(|x| x.len() == 3).unwrap().to_string();
        decoded[8] = knowns.iter().find(|x| x.len() == 7).unwrap().to_string();

        decoded[3] = unknowns
            .iter()
            .filter(|x| x.len() == 5)
            .find(|x| decoded[1].chars().all(|c| x.contains(c)))
            .unwrap()
            .to_string();

        let four = HashSet::from_iter(decoded[4].chars());

        let (nines, zero_sixes): (Vec<&String>, Vec<&String>) =
            unknowns.iter().filter(|x| x.len() == 6).partition(|x| {
                let candidate = HashSet::from_iter(x.chars());
                let union: HashSet<_> = candidate.union(&four).cloned().collect();
                union.is_subset(&candidate)
            });

        assert!(nines.len() == 1);
        assert!(zero_sixes.len() == 2);

        decoded[9] = nines.get(0).unwrap().to_string();

        let (threes, two_fives): (Vec<&String>, Vec<&String>) =
            unknowns.iter().filter(|x| x.len() == 5).partition(|x| {
                let candidate = HashSet::from_iter(x.chars());
                let union: HashSet<_> = candidate.union(&one).cloned().collect();
                union.is_subset(&candidate)
            });

        assert!(threes.len() == 1);
        assert!(two_fives.len() == 2);

        decoded[3] = threes.get(0).unwrap().to_string();

        let nine: HashSet<char> = HashSet::from_iter(decoded[9].chars());
        let eight: HashSet<char> = HashSet::from_iter(decoded[8].chars());
        let ee = eight.difference(&nine).cloned().collect();

        let (twos, fives): (Vec<&String>, Vec<&String>) = two_fives.iter().partition(|x| {
            let candidate = HashSet::from_iter(x.chars());
            let union: HashSet<_> = candidate.union(&ee).cloned().collect();
            union.is_subset(&candidate)
        });

        assert!(twos.len() == 1);
        assert!(fives.len() == 1);

        decoded[2] = twos.get(0).unwrap().to_string();
        decoded[5] = fives.get(0).unwrap().to_string();

        let five: HashSet<char> = HashSet::from_iter(decoded[5].chars());
        let cc = nine.difference(&five).cloned().collect();

        let (zeros, sixes): (Vec<&String>, Vec<&String>) = zero_sixes.iter().partition(|x| {
            let candidate = HashSet::from_iter(x.chars());
            let union: HashSet<_> = candidate.union(&cc).cloned().collect();
            union.is_subset(&candidate)
        });

        assert!(zeros.len() == 1);
        assert!(sixes.len() == 1);

        decoded[0] = zeros.get(0).unwrap().to_string();
        decoded[6] = sixes.get(0).unwrap().to_string();

        let mut out_str = String::new();
        for digit in output {
            let digit_hash: HashSet<char> = HashSet::from_iter(digit.chars());

            let i = decoded
                .iter()
                .position(|x| {
                    let x_hash: HashSet<char> = HashSet::from_iter(x.chars());
                    x_hash.is_subset(&digit_hash) && x_hash.is_superset(&digit_hash)
                })
                .unwrap();

            out_str.push_str(&i.to_string());
        }

        sum += out_str.parse::<i32>().unwrap();
    }

    sum
}

#[test]
fn day8_part2_test() {
    let v = vec![
        String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
        String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
        String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
        String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
        String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
        String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
        String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
        String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
        String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
        String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
    ];
    let answer = day8_part2(v.into_iter());

    //assert_eq!(8394, answer);
    assert_eq!(61229, answer);
}
