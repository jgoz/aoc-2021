use std::{collections::HashSet, env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day11_part1(values)),
        "2" => println!("{}", day11_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

fn for_adjacent<F: FnMut((usize, usize))>((i, j): (usize, usize), len: usize, mut func: F) {
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && (ni as usize) < len && nj >= 0 && (nj as usize) < len {
                func((ni as usize, nj as usize));
            }
        }
    }
}

type Grid = [[u32; 10]; 10];
type Flashed = HashSet<(usize, usize)>;

fn first_pass(oct: &mut Grid) -> Flashed {
    let mut flashed = HashSet::new();
    for i in 0..10 {
        for j in 0..10 {
            oct[i][j] += 1;
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            if oct[i][j] > 9 {
                flash(oct, &mut flashed, (i, j));
            }
        }
    }
    flashed
}

fn flash(oct: &mut Grid, flashed: &mut Flashed, coord: (usize, usize)) {
    let (i, j) = coord;
    if i > 9 || j > 9 || flashed.contains(&coord) {
        return;
    }

    oct[i][j] += 1;

    if oct[i][j] > 9 {
        flashed.insert(coord);

        for_adjacent(coord, 10, |a| flash(oct, flashed, a));
    }
}

fn flash_until_stable(oct: &mut Grid, flashed: &mut Flashed) -> usize {
    let mut last_flashed = flashed.len();
    loop {
        for coord in flashed.clone() {
            flash(oct, flashed, coord);
        }
        if flashed.len() == last_flashed {
            break;
        }
        last_flashed = flashed.len();
    }
    for (i, j) in flashed.clone() {
        oct[i][j] = 0;
    }
    last_flashed
}

fn day11_part1(v: impl Iterator<Item = String>) -> usize {
    let mut oct: Grid = [[0; 10]; 10];

    for (i, line) in v.enumerate() {
        for (j, c) in line.chars().enumerate() {
            oct[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut total_flashed = 0;

    for _ in 0..100 {
        let mut flashed = first_pass(&mut oct);
        let last_flashed = flash_until_stable(&mut oct, &mut flashed);

        total_flashed += last_flashed;
    }

    total_flashed
}

#[test]
fn day11_part1_test() {
    let v = vec![
        String::from("5483143223"),
        String::from("2745854711"),
        String::from("5264556173"),
        String::from("6141336146"),
        String::from("6357385478"),
        String::from("4167524645"),
        String::from("2176841721"),
        String::from("6882881134"),
        String::from("4846848554"),
        String::from("5283751526"),
    ];
    let answer = day11_part1(v.into_iter());

    assert_eq!(1656, answer);
}

fn day11_part2(v: impl Iterator<Item = String>) -> usize {
    let mut oct: Grid = [[0; 10]; 10];

    for (i, line) in v.enumerate() {
        for (j, c) in line.chars().enumerate() {
            oct[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut step = 0;

    loop {
        let mut flashed = first_pass(&mut oct);
        let last_flashed = flash_until_stable(&mut oct, &mut flashed);

        step += 1;

        if last_flashed == 100 {
            break;
        }
    }

    step
}

#[test]
fn day11_part2_test() {
    let v = vec![
        String::from("5483143223"),
        String::from("2745854711"),
        String::from("5264556173"),
        String::from("6141336146"),
        String::from("6357385478"),
        String::from("4167524645"),
        String::from("2176841721"),
        String::from("6882881134"),
        String::from("4846848554"),
        String::from("5283751526"),
    ];
    let answer = day11_part2(v.into_iter());

    assert_eq!(195, answer);
}
