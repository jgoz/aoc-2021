use std::{collections::HashSet, env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day13_part1(values)),
        "2" => day13_part2(values),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Dot(usize, usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Fold(usize, usize);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Paper {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
    max_x: usize,
    max_y: usize,
}

impl Paper {
    fn from(mut v: impl Iterator<Item = String>) -> Self {
        let mut dots = vec![];
        let mut max_x = 0;
        let mut max_y = 0;

        loop {
            let line = v.next().unwrap();
            if line.is_empty() {
                break;
            }

            let (x, y) = line
                .split_once(",")
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap();

            dots.push(Dot(x, y));

            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }

        let mut folds = vec![];
        for line in v {
            let clean = line.replace("fold along ", "");
            let (axis, num) = clean.split_once("=").unwrap();
            match axis {
                "x" => folds.push(Fold(num.parse::<usize>().unwrap(), 0)),
                "y" => folds.push(Fold(0, num.parse::<usize>().unwrap())),
                _ => panic!("Invalid axis"),
            }
        }

        Paper {
            dots,
            folds,
            max_x,
            max_y,
        }
    }

    fn fold(&mut self, times: usize) {
        let mut folds = self.folds.clone().into_iter();

        for _ in 0..times {
            let fold = folds.next();
            if fold.is_none() {
                break;
            }

            let Fold(x, y) = fold.unwrap();

            for dot in self.dots.iter_mut() {
                if x > 0 {
                    // fold along x
                    if dot.0 >= x {
                        let dx = dot.0 - x;
                        dot.0 = x - dx;
                    }
                } else if y > 0 {
                    // fold along y
                    if dot.1 >= y {
                        let dy = dot.1 - y;
                        dot.1 = y - dy;
                    }
                }
            }
        }

        let dots = self.dots.clone();
        let dots_new: HashSet<Dot> = HashSet::from_iter(dots.into_iter());

        self.dots = dots_new.into_iter().collect::<Vec<_>>();
    }

    fn print(&self) {
        // clear
        print!("{esc}[2J", esc = 27 as char);

        // write dots using ansi escape codes
        for Dot(x, y) in self.dots.iter() {
            print!("{esc}[{y};{x}f#", esc = 27 as char, x = x + 1, y = y + 1);
        }

        println!();
        println!();
    }
}

fn day13_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut paper = Paper::from(v);

    paper.fold(1);
    paper.dots.len() as i32
}

#[test]
fn day13_part1_test() {
    let v = vec![
        String::from("6,10"),
        String::from("0,14"),
        String::from("9,10"),
        String::from("0,3"),
        String::from("10,4"),
        String::from("4,11"),
        String::from("6,0"),
        String::from("6,12"),
        String::from("4,1"),
        String::from("0,13"),
        String::from("10,12"),
        String::from("3,4"),
        String::from("3,0"),
        String::from("8,4"),
        String::from("1,10"),
        String::from("2,14"),
        String::from("8,10"),
        String::from("9,0"),
        String::from(""),
        String::from("fold along y=7"),
        String::from("fold along x=5"),
    ];
    let answer = day13_part1(v.into_iter());

    assert_eq!(17, answer);
}

fn day13_part2(v: impl Iterator<Item = String>) {
    let mut paper = Paper::from(v);

    paper.fold(paper.folds.len());
    paper.print();
}
