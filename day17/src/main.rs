use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day17_part1(values)),
        "2" => println!("{}", day17_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone, Copy)]
struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn from(input: String) -> Target {
        let (x_range, y_range) = input.split_once(", ").unwrap();
        let x_string = x_range.to_string().replace("x=", "");
        let (x_min, x_max) = x_string.split_once("..").unwrap();
        let y_string = y_range.to_string().replace("y=", "");
        let (y_min, y_max) = y_string.split_once("..").unwrap();
        Target {
            x_min: x_min.parse::<i32>().unwrap(),
            x_max: x_max.parse::<i32>().unwrap(),
            y_min: y_min.parse::<i32>().unwrap(),
            y_max: y_max.parse::<i32>().unwrap(),
        }
    }

    fn check_point(&self, x: i32, y: i32) -> ShotResult {
        if x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max {
            ShotResult::Hit
        } else {
            if x > self.x_max || y < self.y_min {
                if x >= self.x_min {
                    ShotResult::Miss(MissType::Overshot)
                } else {
                    ShotResult::Miss(MissType::Undershot)
                }
            } else {
                ShotResult::OnTarget
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum MissType {
    Overshot,
    Undershot,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ShotResult {
    Hit,
    Miss(MissType),
    OnTarget,
}

#[derive(Debug, Clone)]
struct Shot {
    x: i32,
    y: i32,
    path: Vec<(i32, i32)>,
    result: ShotResult,
    steps: i32,
}

fn trajectory(vel: (i32, i32), target: &Target) -> Shot {
    let mut shot = Shot {
        x: vel.0,
        y: vel.1,
        path: Vec::new(),
        result: ShotResult::OnTarget,
        steps: 0,
    };
    let mut x = 0;
    let mut y = 0;
    let mut vel_x = vel.0;
    let mut vel_y = vel.1;

    while shot.result == ShotResult::OnTarget {
        x += vel_x;
        y += vel_y;

        shot.path.push((x, y));
        shot.steps += 1;

        if vel_x > 0 {
            vel_x -= 1;
        } else if vel_x < 0 {
            vel_x += 1;
        }

        vel_y -= 1;

        shot.result = target.check_point(x, y);
    }

    shot
}

fn fire_shots_from_y(vel_y: i32, target: &Target) -> Vec<Shot> {
    let mut hits = Vec::new();
    let mut vel_x = 1;

    loop {
        let shot = trajectory((vel_x, vel_y), target);
        let steps = shot.steps;

        if steps == 1 && shot.result == ShotResult::Miss(MissType::Overshot) {
            // Stop when our first step is an overshot
            break;
        }
        if shot.result == ShotResult::Hit {
            hits.push(shot);
        }

        vel_x += 1;
    }

    hits
}

fn fire_shots(target: &Target) -> Vec<Shot> {
    let mut hits = Vec::new();

    // Fire shots until the y velocity is such that it will
    // never hit the target
    for vel_y in target.y_min..-target.y_min {
        hits.extend(fire_shots_from_y(vel_y, target));
    }

    hits
}

fn day17_part1(mut v: impl Iterator<Item = String>) -> i32 {
    let input = v.next().unwrap().replace("target area: ", "");
    let target = Target::from(input);

    let hits = fire_shots(&target);

    let max_y = hits
        .iter()
        .map(|x| x.path.iter().map(|p| p.1).max().unwrap_or(0))
        .max()
        .unwrap();

    max_y
}

#[test]
fn day17_part1_test() {
    let v = vec![String::from("target area: x=20..30, y=-10..-5")];
    let answer = day17_part1(v.into_iter());

    assert_eq!(45, answer);
}

fn day17_part2(mut v: impl Iterator<Item = String>) -> i32 {
    let input = v.next().unwrap().replace("target area: ", "");
    let target = Target::from(input);

    let hits = fire_shots(&target);

    hits.len() as i32
}

#[test]
fn day17_part2_test() {
    let v = vec![String::from("target area: x=20..30, y=-10..-5")];
    let answer = day17_part2(v.into_iter());

    assert_eq!(112, answer);
}
