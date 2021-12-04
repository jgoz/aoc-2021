use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day4_part1(values)),
        "2" => println!("{}", day4_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Copy, Clone, Debug)]
struct Space {
    num: i32,
    marked: bool,
}

impl Space {
    fn new(num: i32) -> Space {
        Space { num, marked: false }
    }
    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Copy, Clone, Debug)]
struct Board {
    rows: [[Space; 5]; 5],
    cols: [[Space; 5]; 5],
    has_match: bool,
}

impl Board {
    fn new(nums: Vec<i32>) -> Board {
        let rows = (0..5)
            .into_iter()
            .map(|r| {
                [
                    Space::new(nums[r * 5]),
                    Space::new(nums[r * 5 + 1]),
                    Space::new(nums[r * 5 + 2]),
                    Space::new(nums[r * 5 + 3]),
                    Space::new(nums[r * 5 + 4]),
                ]
            })
            .collect::<Vec<_>>();

        let cols = (0..5)
            .into_iter()
            .map(|c| {
                [
                    Space::new(nums[c]),
                    Space::new(nums[5 + c]),
                    Space::new(nums[10 + c]),
                    Space::new(nums[15 + c]),
                    Space::new(nums[20 + c]),
                ]
            })
            .collect::<Vec<_>>();

        Board {
            rows: [rows[0], rows[1], rows[2], rows[3], rows[4]],
            cols: [cols[0], cols[1], cols[2], cols[3], cols[4]],
            has_match: false,
        }
    }

    fn stamp(&mut self, num: i32) {
        for row in self.rows.iter_mut() {
            for space in row {
                if space.num == num {
                    space.mark();
                }
            }
        }
        for col in self.cols.iter_mut() {
            for space in col {
                if space.num == num {
                    space.mark();
                }
            }
        }
    }

    fn check(self) -> bool {
        for row in self.rows.iter() {
            if row.iter().all(|x| x.marked) {
                return true;
            }
        }
        for col in self.cols.iter() {
            if col.iter().all(|x| x.marked) {
                return true;
            }
        }
        false
    }

    fn unstamped(self) -> i32 {
        self.rows
            .iter()
            .flat_map(|r| {
                r.iter()
                    .filter_map(|x| if !x.marked { Some(x.num) } else { None })
            })
            .sum()
    }

    fn mark(&mut self) {
        self.has_match = true;
    }
}

fn day4_part1(mut v: impl Iterator<Item = String>) -> i32 {
    let mut boards: Vec<Board> = vec![];

    let first_line = v.next().unwrap();
    let numbers = first_line
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut board_nums: Vec<i32> = vec![];

    v.next(); // Skip the first line

    for line in v {
        if line.len() == 0 {
            continue;
        }

        for num_str in line.split(" ") {
            if let Ok(num) = num_str.parse::<i32>() {
                board_nums.push(num)
            }
        }

        if board_nums.len() == 25 {
            boards.push(Board::new(board_nums));
            board_nums = vec![];
        }
    }

    for num in numbers.into_iter() {
        for board in boards.iter_mut() {
            board.stamp(num);
            if board.check() {
                let unstamped = board.unstamped();
                return unstamped * num;
            }
        }
    }

    0
}

#[test]
fn day4_part1_test() {
    let v = vec![
        String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
        String::from(""),
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from(" 1 12 20 15 19"),
        String::from(""),
        String::from(" 3 15  0  2 22"),
        String::from(" 9 18 13 17  5"),
        String::from("19  8  7 25 23"),
        String::from("20 11 10 24  4"),
        String::from("14 21 16 12  6"),
        String::from(""),
        String::from("14 21 17 24  4"),
        String::from("10 16 15  9 19"),
        String::from("18  8 23 26 20"),
        String::from("22 11 13  6  5"),
        String::from(" 2  0 12  3  7"),
    ];
    let answer = day4_part1(v.into_iter());

    assert_eq!(4512, answer);
}

fn day4_part2(mut v: impl Iterator<Item = String>) -> i32 {
    let mut boards: Vec<Board> = vec![];

    let first_line = v.next().unwrap();
    let numbers = first_line
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut board_nums: Vec<i32> = vec![];

    v.next(); // Skip the first line

    for line in v {
        if line.len() == 0 {
            continue;
        }

        for num_str in line.split(" ") {
            if let Ok(num) = num_str.parse::<i32>() {
                board_nums.push(num)
            }
        }

        if board_nums.len() == 25 {
            boards.push(Board::new(board_nums));
            board_nums = vec![];
        }
    }

    let mut matches = 0;
    let total = boards.len();

    for num in numbers.into_iter() {
        for board in boards.iter_mut() {
            board.stamp(num);
            if !board.has_match && board.check() {
                board.mark();
                matches += 1;
                if matches == total {
                    let unstamped = board.unstamped();
                    return unstamped * num;
                }
            }
        }
    }

    0
}

#[test]
fn day4_part2_test() {
    let v = vec![
        String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
        String::from(""),
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from(" 1 12 20 15 19"),
        String::from(""),
        String::from(" 3 15  0  2 22"),
        String::from(" 9 18 13 17  5"),
        String::from("19  8  7 25 23"),
        String::from("20 11 10 24  4"),
        String::from("14 21 16 12  6"),
        String::from(""),
        String::from("14 21 17 24  4"),
        String::from("10 16 15  9 19"),
        String::from("18  8 23 26 20"),
        String::from("22 11 13  6  5"),
        String::from(" 2  0 12  3  7"),
    ];
    let answer = day4_part2(v.into_iter());

    assert_eq!(1924, answer);
}
