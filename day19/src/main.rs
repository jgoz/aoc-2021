use rayon::prelude::*;
use std::{collections::HashSet, env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day19_part1(values)),
        "2" => println!("{}", day19_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

static SWAPS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 1, 0],
    [2, 0, 1],
];

static FLIPS: [[i32; 3]; 8] = [
    [1, 1, 1],
    [-1, 1, 1],
    [1, -1, 1],
    [1, 1, -1],
    [1, -1, -1],
    [-1, -1, 1],
    [-1, 1, -1],
    [-1, -1, -1],
];

type Pos = [i32; 3];

#[derive(Debug, Clone)]
struct Scanner {
    id: i32,
    pos: Pos,
    beacons: Vec<Pos>,
}

fn rel_magnitudes(beacons: &Vec<Pos>, beacon: &Pos) -> Vec<(usize, Pos)> {
    beacons
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            (
                i,
                [
                    (beacon[X] - pos[X]).abs(),
                    (beacon[Y] - pos[Y]).abs(),
                    (beacon[Z] - pos[Z]).abs(),
                ],
            )
        })
        .collect::<Vec<_>>()
}

impl Scanner {
    fn from(mut v: impl Iterator<Item = String>) -> Scanner {
        let id = v
            .next()
            .unwrap()
            .to_string()
            .strip_prefix("--- scanner ")
            .unwrap()
            .strip_suffix(" ---")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();

        let mut beacons = Vec::new();
        while let Some(line) = v.next() {
            if line.len() == 0 {
                break;
            }
            let pos = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            beacons.push([pos[0], pos[1], pos[2]]);
        }

        Scanner {
            id,
            beacons,
            // Guesses
            pos: [0, 0, 0],
        }
    }

    fn transpose_onto(&self, other: &Scanner) -> Option<Scanner> {
        self.beacons.par_iter().find_map_any(|&beacon| {
            let relative_magnitudes_self = rel_magnitudes(&self.beacons, &beacon);

            for flip in FLIPS {
                for swap in SWAPS {
                    let translated = other.translate(flip, swap);

                    let scanner = translated.par_iter().find_map_any(|&other_beacon| {
                        let mut overlaps = Vec::new();

                        let relative_magnitudes_other = rel_magnitudes(&translated, &other_beacon);

                        for (j, mag) in relative_magnitudes_other.iter() {
                            if let Some((i, _)) = relative_magnitudes_self
                                .iter()
                                .find(|(_, other)| *mag == *other)
                            {
                                overlaps.push((*i, *j));
                            }
                        }

                        if overlaps.len() >= 12 {
                            let mut offsets = overlaps.iter().map(|(i, j)| {
                                [
                                    (self.beacons[*i][0] - translated[*j][0]),
                                    (self.beacons[*i][1] - translated[*j][1]),
                                    (self.beacons[*i][2] - translated[*j][2]),
                                ]
                            });

                            let offset_0 = offsets.next().unwrap();
                            if offsets.skip(1).all(|o| o == offset_0) {
                                let beacons = translated
                                    .iter()
                                    .map(|b| {
                                        [b[0] + offset_0[0], b[1] + offset_0[1], b[2] + offset_0[2]]
                                    })
                                    .collect();

                                return Some(Scanner {
                                    id: other.id,
                                    pos: offset_0,
                                    beacons,
                                });
                            }
                        }

                        None
                    });

                    if scanner.is_some() {
                        return scanner;
                    }
                }
            }
            None
        })
    }

    fn translate(&self, flip: [i32; 3], swap: [usize; 3]) -> Vec<Pos> {
        self.beacons
            .iter()
            .map(|pos| {
                [
                    flip[X] * pos[swap[X]],
                    flip[Y] * pos[swap[Y]],
                    flip[Z] * pos[swap[Z]],
                ]
            })
            .collect()
    }
}

fn transpose_all_scanners(v: impl Iterator<Item = String>) -> Vec<Scanner> {
    let lines = v.collect::<Vec<_>>();
    let scanner_lines = lines.split(|x| x.is_empty()).collect::<Vec<_>>();
    let scanners = scanner_lines
        .iter()
        .map(|lines| Scanner::from(lines.into_iter().cloned()))
        .collect::<Vec<_>>();

    let mut rotated = vec![scanners[0].clone()];

    let mut tried = HashSet::new();

    while rotated.len() < scanners.len() {
        let cur_rot = rotated.clone();

        'found: for scanner in cur_rot.iter() {
            for other in scanners.iter() {
                if tried.contains(&(scanner.id, other.id))
                    || cur_rot.iter().any(|s| s.id == other.id)
                {
                    continue;
                }

                let next = scanner.transpose_onto(other);
                tried.insert((scanner.id, other.id));
                if next.is_some() {
                    println!("{} {}", scanner.id.to_string(), other.id.to_string());
                    rotated.push(next.unwrap());
                    continue 'found;
                }
            }
        }
    }

    rotated.sort_by_key(|s| s.id);
    rotated
}

fn day19_part1(v: impl Iterator<Item = String>) -> i32 {
    let rotated = transpose_all_scanners(v);

    let mut beacons: HashSet<Pos> = HashSet::new();
    for scanner in rotated.into_iter() {
        beacons.extend(scanner.beacons.iter().cloned());
    }

    beacons.len() as i32
}

#[test]
fn day19_part1_test() {
    let v = get_test_input();
    let answer = day19_part1(v.into_iter());

    assert_eq!(79, answer);
}

fn day19_part2(v: impl Iterator<Item = String>) -> i32 {
    let rotated = transpose_all_scanners(v);
    let mut max_dist = 0;

    for a in rotated.iter() {
        for b in rotated.iter() {
            let dist = (a.pos[X] - b.pos[X]).abs()
                + (a.pos[Y] - b.pos[Y]).abs()
                + (a.pos[Z] - b.pos[Z]).abs();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    max_dist
}

#[test]
fn day19_part2_test() {
    let v = get_test_input();
    let answer = day19_part2(v.into_iter());

    assert_eq!(3621, answer);
}

#[test]
fn overlaps_test() {
    let v = get_test_input();
    let lines = v.split(|x| x.is_empty()).collect::<Vec<_>>();

    let scanner_0 = Scanner::from(lines[0].clone().into_iter().cloned());
    let scanner_1 = Scanner::from(lines[1].clone().into_iter().cloned());
    let scanner_2 = Scanner::from(lines[2].clone().into_iter().cloned());
    let scanner_4 = Scanner::from(lines[4].clone().into_iter().cloned());

    let rotated = scanner_0.transpose_onto(&scanner_1).unwrap();

    assert_eq!(rotated.pos, [68, -1246, -43]);

    let rotated_2 = rotated.transpose_onto(&scanner_4).unwrap();

    assert_eq!(rotated_2.pos, [-20, -1133, 1061]);

    let rotated_3 = rotated_2.transpose_onto(&scanner_2).unwrap();
    assert_eq!(rotated_3.pos, [1105, -1205, 1229]);
}

#[allow(unused)]
fn get_test_input() -> Vec<String> {
    return vec![
        String::from("--- scanner 0 ---"),
        String::from("404,-588,-901"),
        String::from("528,-643,409"),
        String::from("-838,591,734"),
        String::from("390,-675,-793"),
        String::from("-537,-823,-458"),
        String::from("-485,-357,347"),
        String::from("-345,-311,381"),
        String::from("-661,-816,-575"),
        String::from("-876,649,763"),
        String::from("-618,-824,-621"),
        String::from("553,345,-567"),
        String::from("474,580,667"),
        String::from("-447,-329,318"),
        String::from("-584,868,-557"),
        String::from("544,-627,-890"),
        String::from("564,392,-477"),
        String::from("455,729,728"),
        String::from("-892,524,684"),
        String::from("-689,845,-530"),
        String::from("423,-701,434"),
        String::from("7,-33,-71"),
        String::from("630,319,-379"),
        String::from("443,580,662"),
        String::from("-789,900,-551"),
        String::from("459,-707,401"),
        String::from(""),
        String::from("--- scanner 1 ---"),
        String::from("686,422,578"),
        String::from("605,423,415"),
        String::from("515,917,-361"),
        String::from("-336,658,858"),
        String::from("95,138,22"),
        String::from("-476,619,847"),
        String::from("-340,-569,-846"),
        String::from("567,-361,727"),
        String::from("-460,603,-452"),
        String::from("669,-402,600"),
        String::from("729,430,532"),
        String::from("-500,-761,534"),
        String::from("-322,571,750"),
        String::from("-466,-666,-811"),
        String::from("-429,-592,574"),
        String::from("-355,545,-477"),
        String::from("703,-491,-529"),
        String::from("-328,-685,520"),
        String::from("413,935,-424"),
        String::from("-391,539,-444"),
        String::from("586,-435,557"),
        String::from("-364,-763,-893"),
        String::from("807,-499,-711"),
        String::from("755,-354,-619"),
        String::from("553,889,-390"),
        String::from(""),
        String::from("--- scanner 2 ---"),
        String::from("649,640,665"),
        String::from("682,-795,504"),
        String::from("-784,533,-524"),
        String::from("-644,584,-595"),
        String::from("-588,-843,648"),
        String::from("-30,6,44"),
        String::from("-674,560,763"),
        String::from("500,723,-460"),
        String::from("609,671,-379"),
        String::from("-555,-800,653"),
        String::from("-675,-892,-343"),
        String::from("697,-426,-610"),
        String::from("578,704,681"),
        String::from("493,664,-388"),
        String::from("-671,-858,530"),
        String::from("-667,343,800"),
        String::from("571,-461,-707"),
        String::from("-138,-166,112"),
        String::from("-889,563,-600"),
        String::from("646,-828,498"),
        String::from("640,759,510"),
        String::from("-630,509,768"),
        String::from("-681,-892,-333"),
        String::from("673,-379,-804"),
        String::from("-742,-814,-386"),
        String::from("577,-820,562"),
        String::from(""),
        String::from("--- scanner 3 ---"),
        String::from("-589,542,597"),
        String::from("605,-692,669"),
        String::from("-500,565,-823"),
        String::from("-660,373,557"),
        String::from("-458,-679,-417"),
        String::from("-488,449,543"),
        String::from("-626,468,-788"),
        String::from("338,-750,-386"),
        String::from("528,-832,-391"),
        String::from("562,-778,733"),
        String::from("-938,-730,414"),
        String::from("543,643,-506"),
        String::from("-524,371,-870"),
        String::from("407,773,750"),
        String::from("-104,29,83"),
        String::from("378,-903,-323"),
        String::from("-778,-728,485"),
        String::from("426,699,580"),
        String::from("-438,-605,-362"),
        String::from("-469,-447,-387"),
        String::from("509,732,623"),
        String::from("647,635,-688"),
        String::from("-868,-804,481"),
        String::from("614,-800,639"),
        String::from("595,780,-596"),
        String::from(""),
        String::from("--- scanner 4 ---"),
        String::from("727,592,562"),
        String::from("-293,-554,779"),
        String::from("441,611,-461"),
        String::from("-714,465,-776"),
        String::from("-743,427,-804"),
        String::from("-660,-479,-426"),
        String::from("832,-632,460"),
        String::from("927,-485,-438"),
        String::from("408,393,-506"),
        String::from("466,436,-512"),
        String::from("110,16,151"),
        String::from("-258,-428,682"),
        String::from("-393,719,612"),
        String::from("-211,-452,876"),
        String::from("808,-476,-593"),
        String::from("-575,615,604"),
        String::from("-485,667,467"),
        String::from("-680,325,-822"),
        String::from("-627,-443,-432"),
        String::from("872,-547,-609"),
        String::from("833,512,582"),
        String::from("807,604,487"),
        String::from("839,-516,451"),
        String::from("891,-625,532"),
        String::from("-652,-548,-490"),
        String::from("30,-46,-14"),
    ];
}
