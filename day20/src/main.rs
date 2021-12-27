use std::{env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day20_part1(values)),
        "2" => println!("{}", day20_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

struct Image {
    alg: Vec<char>,
    bits: Vec<char>,
    width: isize,
    height: isize,
    assume_zero: bool,
}

impl Image {
    fn from(mut v: impl Iterator<Item = String>) -> Image {
        let alg_str = v.next().unwrap();
        v.next();

        let mut width = 0;
        let mut height = 0;

        let mut bits = String::new();
        while let Some(line) = v.next() {
            if line.is_empty() {
                continue;
            }
            width = line.len() as isize;
            height += 1;

            bits.push_str(&line);
        }

        let alg = alg_str.chars().collect();

        Image {
            alg,
            bits: bits.chars().collect(),
            width,
            height,
            assume_zero: false,
        }
    }

    fn get_input_pixel(&self, x: isize, y: isize) -> char {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            // Handle the "infinite" case by returning the "zero" value if
            // requested. This will happen if zero is "#" which will cause the
            // border to flash. Rather than rendering a solid border out to infinity,
            // we can just assume it will be set to this when calculating the inner values.
            return if self.assume_zero { self.alg[0] } else { '.' };
        }

        let idx = (y * self.width + x) as usize;
        self.bits[idx]
    }

    fn get_output_pixel(&self, x: isize, y: isize) -> char {
        let mut index = 0;
        let mut i = 8;

        for iy in y - 1..=y + 1 {
            for ix in x - 1..=x + 1 {
                let pixel = self.get_input_pixel(ix, iy);
                index = index
                    | match pixel {
                        '.' => 0,
                        '#' => 1 << i,
                        _ => panic!("Invalid pixel {}", pixel),
                    };
                i -= 1;
            }
        }

        self.alg[index]
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                out.push(self.bits[(y * self.width + x) as usize]);
            }
            out.push('\n');
        }
        out
    }

    pub fn enhance(&self) -> Image {
        let mut bits = Vec::new();
        for y in -1..self.height + 1 {
            for x in -1..self.width + 1 {
                bits.push(self.get_output_pixel(x, y));
            }
        }

        Image {
            alg: self.alg.clone(),
            bits,
            width: self.width + 2,
            height: self.height + 2,
            // This inverts with each iteration because that's how the flashing
            // border would work if alg[0] is '#'
            assume_zero: !self.assume_zero,
        }
    }
}

fn day20_part1(v: impl Iterator<Item = String>) -> i32 {
    let image0 = Image::from(v);
    //print!("0:\n{}\n\n", image0.to_string());
    let image1 = image0.enhance();
    //print!("1:\n{}\n\n", image1.to_string());
    let image2 = image1.enhance();
    //print!("2:\n{}\n\n", image2.to_string());

    image2.bits.iter().filter(|&&c| c == '#').count() as i32
}

#[test]
fn day20_part1_test() {
    let v = vec![
        String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"),
        String::from(""),
        String::from("#..#."),
        String::from("#...."),
        String::from("##..#"),
        String::from("..#.."),
        String::from("..###"),
    ];
    let answer = day20_part1(v.into_iter());

    assert_eq!(35, answer);
}

fn day20_part2(v: impl Iterator<Item = String>) -> i32 {
    let mut image = Image::from(v);

    for _ in 0..50 {
        image = image.enhance();
    }

    image.bits.iter().filter(|&&c| c == '#').count() as i32
}

#[test]
fn day20_part2_test() {
    let v = vec![
        String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"),
        String::from(""),
        String::from("#..#."),
        String::from("#...."),
        String::from("##..#"),
        String::from("..#.."),
        String::from("..###"),
    ];
    let answer = day20_part2(v.into_iter());

    assert_eq!(3351, answer);
}
