use std::{collections::HashMap, env, hash::Hash, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day21_part1(values)),
        "2" => println!("{}", day21_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

struct Player {
    space: u32,
    score: u32,
}

impl Player {
    fn roll(&mut self, die_roller: &mut impl Iterator<Item = u32>, rolls: &mut u32) {
        for _ in 0..3 {
            if let Some(roll) = die_roller.next() {
                self.space += roll;
            }
            *rolls += 1;
        }
        while self.space > 10 {
            self.space -= 10;
        }
        self.score += self.space;
    }
}

fn day21_part1(v: impl Iterator<Item = String>) -> u32 {
    let mut die_roller = (1..=100).cycle();
    let mut rolls = 0;

    let mut players = v
        .map(|str| {
            let space = str.split_once(": ").unwrap().1.parse::<u32>().unwrap();
            Player { space, score: 0 }
        })
        .collect::<Vec<_>>();

    'game: loop {
        for player in players.iter_mut() {
            player.roll(&mut die_roller, &mut rolls);
            if player.score >= 1000 {
                break 'game;
            }
        }
    }

    let loser = players.iter().find(|p| p.score < 1000).unwrap();

    loser.score * rolls
}

#[test]
fn day21_part1_test() {
    let v = vec![
        String::from("Player 1 starting position: 4"),
        String::from("Player 2 starting position: 8"),
    ];
    let answer = day21_part1(v.into_iter());

    assert_eq!(739785, answer);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Turn {
    spaces: [u32; 2],
    scores: [u32; 2],
}

impl Turn {
    fn next_turn(&self, i: usize, r1: u32, r2: u32, r3: u32) -> Turn {
        let mut spaces = self.spaces.clone();
        spaces[i] += r1 + r2 + r3;

        if spaces[i] > 10 {
            spaces[i] -= 10;
        }

        let mut scores = self.scores.clone();
        scores[i] += spaces[i];

        Self { spaces, scores }
    }

    fn is_win(&self, i: usize) -> bool {
        self.scores[i] >= 21
    }
}

fn day21_part2(v: impl Iterator<Item = String>) -> u64 {
    let spaces = v
        .map(|str| str.split_once(": ").unwrap().1.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let rolls = (1..=3)
        .map(|r1| (1..=3).map(move |r2| (1..=3).map(move |r3| (r1, r2, r3))))
        .flatten()
        .flatten();

    let mut turns: HashMap<Turn, u64> = HashMap::from([(
        Turn {
            spaces: [spaces[0], spaces[1]],
            scores: [0, 0],
        },
        1,
    )]);

    let mut wins: [u64; 2] = [0, 0];

    while !turns.is_empty() {
        for p in 0..=1 {
            let mut next_turns = HashMap::new();

            for (turn, count) in turns.clone() {
                for (r1, r2, r3) in rolls.clone() {
                    let new_turn = turn.next_turn(p, r1, r2, r3);
                    if new_turn.is_win(p) {
                        wins[p] += count;
                    } else {
                        let new_count = next_turns.entry(new_turn).or_insert(0);
                        *new_count += count;
                    }
                }
            }

            turns = next_turns;
        }
    }

    *wins.iter().max().unwrap()
}

#[test]
fn day21_part2_test() {
    let v = vec![
        String::from("Player 1 starting position: 4"),
        String::from("Player 2 starting position: 8"),
    ];
    let answer = day21_part2(v.into_iter());

    assert_eq!(444356092776315, answer);
}
