use std::{collections::HashMap, env, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day22_part1(values)),
        "2" => println!("{}", day22_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i32, i32, i32);

fn split_range<'a>(s: &'a str, prefix: &'a str) -> (&'a str, &'a str) {
    s.strip_prefix(prefix).unwrap().split_once("..").unwrap()
}

fn day22_part1(v: impl Iterator<Item = String>) -> i32 {
    let mut cubes = HashMap::new();

    for line in v {
        let (on_off, coords) = line.split_once(" ").unwrap();
        let xyzs = coords.split(",").collect::<Vec<_>>();

        let x_range = split_range(xyzs[0], "x=");
        let y_range = split_range(xyzs[1], "y=");
        let z_range = split_range(xyzs[2], "z=");

        let x_min = x_range.0.parse::<i32>().unwrap().max(-50);
        let x_max = x_range.1.parse::<i32>().unwrap().min(50);
        let y_min = y_range.0.parse::<i32>().unwrap().max(-50);
        let y_max = y_range.1.parse::<i32>().unwrap().min(50);
        let z_min = z_range.0.parse::<i32>().unwrap().max(-50);
        let z_max = z_range.1.parse::<i32>().unwrap().min(50);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    let pos = Pos(x, y, z);
                    if on_off == "on" {
                        cubes.insert(pos, true);
                    } else {
                        cubes.remove(&pos);
                    }
                }
            }
        }
    }

    cubes.len() as i32
}

#[test]
fn day22_part1_test() {
    let v = vec![
        String::from("on x=10..12,y=10..12,z=10..12"),
        String::from("on x=11..13,y=11..13,z=11..13"),
        String::from("off x=9..11,y=9..11,z=9..11"),
        String::from("on x=10..10,y=10..10,z=10..10"),
    ];
    let answer = day22_part2(v.into_iter());

    assert_eq!(39, answer);
}

#[test]
fn day22_part1_test_2() {
    let v = vec![
        String::from("on x=-20..26,y=-36..17,z=-47..7"),
        String::from("on x=-20..33,y=-21..23,z=-26..28"),
        String::from("on x=-22..28,y=-29..23,z=-38..16"),
        String::from("on x=-46..7,y=-6..46,z=-50..-1"),
        String::from("on x=-49..1,y=-3..46,z=-24..28"),
        String::from("on x=2..47,y=-22..22,z=-23..27"),
        String::from("on x=-27..23,y=-28..26,z=-21..29"),
        String::from("on x=-39..5,y=-6..47,z=-3..44"),
        String::from("on x=-30..21,y=-8..43,z=-13..34"),
        String::from("on x=-22..26,y=-27..20,z=-29..19"),
        String::from("off x=-48..-32,y=26..41,z=-47..-37"),
        String::from("on x=-12..35,y=6..50,z=-50..-2"),
        String::from("off x=-48..-32,y=-32..-16,z=-15..-5"),
        String::from("on x=-18..26,y=-33..15,z=-7..46"),
        String::from("off x=-40..-22,y=-38..-28,z=23..41"),
        String::from("on x=-16..35,y=-41..10,z=-47..6"),
        String::from("off x=-32..-23,y=11..30,z=-14..3"),
        String::from("on x=-49..-5,y=-3..45,z=-29..18"),
        String::from("off x=18..30,y=-20..-8,z=-3..13"),
        String::from("on x=-41..9,y=-7..43,z=-33..15"),
        String::from("on x=-54112..-39298,y=-85059..-49293,z=-27449..7877"),
        String::from("on x=967..23432,y=45373..81175,z=27513..53682"),
    ];
    let answer = day22_part1(v.into_iter());

    assert_eq!(590784, answer);
}

fn parse_range(range: (&str, &str)) -> (i32, i32) {
    let min = range.0.parse::<i32>().unwrap();
    let max = range.1.parse::<i32>().unwrap();
    (min, max)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
    negatives: Vec<Region>,
}

impl Region {
    fn intersects(&self, other: &Region) -> bool {
        let (min_x1, max_x1) = self.x_range;
        let (min_x2, max_x2) = other.x_range;
        let (min_y1, max_y1) = self.y_range;
        let (min_y2, max_y2) = other.y_range;
        let (min_z1, max_z1) = self.z_range;
        let (min_z2, max_z2) = other.z_range;

        ((min_x1 <= min_x2 && min_x2 <= max_x1) || (min_x2 <= min_x1 && min_x1 <= max_x2))
            && ((min_y1 <= min_y2 && min_y2 <= max_y1) || (min_y2 <= min_y1 && min_y1 <= max_y2))
            && ((min_z1 <= min_z2 && min_z2 <= max_z1) || (min_z2 <= min_z1 && min_z1 <= max_z2))
    }

    fn overlap_with(&mut self, other: &Region) {
        if !self.intersects(other) {
            return;
        }

        let x_range = (
            self.x_range.0.max(other.x_range.0),
            self.x_range.1.min(other.x_range.1),
        );
        let y_range = (
            self.y_range.0.max(other.y_range.0),
            self.y_range.1.min(other.y_range.1),
        );
        let z_range = (
            self.z_range.0.max(other.z_range.0),
            self.z_range.1.min(other.z_range.1),
        );

        let overlap = Region {
            x_range,
            y_range,
            z_range,
            negatives: vec![],
        };

        for negative in self.negatives.iter_mut() {
            // Check for negatives with our negatives, which will produce a positive
            negative.overlap_with(&overlap);
        }

        // Overlaps always produce a negative regardless of whether
        // the other region was on or off. If it was off, we need to
        // turn off the overlapping region; if it was on, we need to
        // subtract it from the current volume to avoid double counting
        // (the overlapping region will be counted after this one).
        self.negatives.push(overlap);
    }

    fn volume(&self) -> i64 {
        let x_len = self.x_range.1 - self.x_range.0 + 1;
        let y_len = self.y_range.1 - self.y_range.0 + 1;
        let z_len = self.z_range.1 - self.z_range.0 + 1;

        let self_volume = x_len as i64 * y_len as i64 * z_len as i64;

        // Subtract overlapping regions from our total volume. Since, this
        // is recursive, if there are overlaps with our overlaps, those volumes
        // will be positive from the perspective of the current cube. This accounts
        // for the situation where a region was turned off, and then turned back on
        // or vice versa.
        let overlapping_volume: i64 = self.negatives.iter().map(|r| r.volume()).sum();
        self_volume - overlapping_volume
    }
}

fn day22_part2(v: impl Iterator<Item = String>) -> i64 {
    let mut regions: Vec<Region> = vec![];

    for line in v {
        let (on_off, coords) = line.split_once(" ").unwrap();
        let xyzs = coords.split(",").collect::<Vec<_>>();

        let (x_min, x_max) = parse_range(split_range(xyzs[0], "x="));
        let (y_min, y_max) = parse_range(split_range(xyzs[1], "y="));
        let (z_min, z_max) = parse_range(split_range(xyzs[2], "z="));

        let region = Region {
            x_range: (x_min, x_max),
            y_range: (y_min, y_max),
            z_range: (z_min, z_max),
            negatives: vec![],
        };

        for other in regions.iter_mut() {
            other.overlap_with(&region);
        }

        if on_off == "on" {
            regions.push(region);
        }
    }

    regions.iter().map(|r| r.volume()).sum()
}

#[test]
fn day22_part2_test() {
    let v = vec![
        String::from("on x=-5..47,y=-31..22,z=-19..33"),
        String::from("on x=-44..5,y=-27..21,z=-14..35"),
        String::from("on x=-49..-1,y=-11..42,z=-10..38"),
        String::from("on x=-20..34,y=-40..6,z=-44..1"),
        String::from("off x=26..39,y=40..50,z=-2..11"),
        String::from("on x=-41..5,y=-41..6,z=-36..8"),
        String::from("off x=-43..-33,y=-45..-28,z=7..25"),
        String::from("on x=-33..15,y=-32..19,z=-34..11"),
        String::from("off x=35..47,y=-46..-34,z=-11..5"),
        String::from("on x=-14..36,y=-6..44,z=-16..29"),
        String::from("on x=-57795..-6158,y=29564..72030,z=20435..90618"),
        String::from("on x=36731..105352,y=-21140..28532,z=16094..90401"),
        String::from("on x=30999..107136,y=-53464..15513,z=8553..71215"),
        String::from("on x=13528..83982,y=-99403..-27377,z=-24141..23996"),
        String::from("on x=-72682..-12347,y=18159..111354,z=7391..80950"),
        String::from("on x=-1060..80757,y=-65301..-20884,z=-103788..-16709"),
        String::from("on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856"),
        String::from("on x=-52752..22273,y=-49450..9096,z=54442..119054"),
        String::from("on x=-29982..40483,y=-108474..-28371,z=-24328..38471"),
        String::from("on x=-4958..62750,y=40422..118853,z=-7672..65583"),
        String::from("on x=55694..108686,y=-43367..46958,z=-26781..48729"),
        String::from("on x=-98497..-18186,y=-63569..3412,z=1232..88485"),
        String::from("on x=-726..56291,y=-62629..13224,z=18033..85226"),
        String::from("on x=-110886..-34664,y=-81338..-8658,z=8914..63723"),
        String::from("on x=-55829..24974,y=-16897..54165,z=-121762..-28058"),
        String::from("on x=-65152..-11147,y=22489..91432,z=-58782..1780"),
        String::from("on x=-120100..-32970,y=-46592..27473,z=-11695..61039"),
        String::from("on x=-18631..37533,y=-124565..-50804,z=-35667..28308"),
        String::from("on x=-57817..18248,y=49321..117703,z=5745..55881"),
        String::from("on x=14781..98692,y=-1341..70827,z=15753..70151"),
        String::from("on x=-34419..55919,y=-19626..40991,z=39015..114138"),
        String::from("on x=-60785..11593,y=-56135..2999,z=-95368..-26915"),
        String::from("on x=-32178..58085,y=17647..101866,z=-91405..-8878"),
        String::from("on x=-53655..12091,y=50097..105568,z=-75335..-4862"),
        String::from("on x=-111166..-40997,y=-71714..2688,z=5609..50954"),
        String::from("on x=-16602..70118,y=-98693..-44401,z=5197..76897"),
        String::from("on x=16383..101554,y=4615..83635,z=-44907..18747"),
        String::from("off x=-95822..-15171,y=-19987..48940,z=10804..104439"),
        String::from("on x=-89813..-14614,y=16069..88491,z=-3297..45228"),
        String::from("on x=41075..99376,y=-20427..49978,z=-52012..13762"),
        String::from("on x=-21330..50085,y=-17944..62733,z=-112280..-30197"),
        String::from("on x=-16478..35915,y=36008..118594,z=-7885..47086"),
        String::from("off x=-98156..-27851,y=-49952..43171,z=-99005..-8456"),
        String::from("off x=2032..69770,y=-71013..4824,z=7471..94418"),
        String::from("on x=43670..120875,y=-42068..12382,z=-24787..38892"),
        String::from("off x=37514..111226,y=-45862..25743,z=-16714..54663"),
        String::from("off x=25699..97951,y=-30668..59918,z=-15349..69697"),
        String::from("off x=-44271..17935,y=-9516..60759,z=49131..112598"),
        String::from("on x=-61695..-5813,y=40978..94975,z=8655..80240"),
        String::from("off x=-101086..-9439,y=-7088..67543,z=33935..83858"),
        String::from("off x=18020..114017,y=-48931..32606,z=21474..89843"),
        String::from("off x=-77139..10506,y=-89994..-18797,z=-80..59318"),
        String::from("off x=8476..79288,y=-75520..11602,z=-96624..-24783"),
        String::from("on x=-47488..-1262,y=24338..100707,z=16292..72967"),
        String::from("off x=-84341..13987,y=2429..92914,z=-90671..-1318"),
        String::from("off x=-37810..49457,y=-71013..-7894,z=-105357..-13188"),
        String::from("off x=-27365..46395,y=31009..98017,z=15428..76570"),
        String::from("off x=-70369..-16548,y=22648..78696,z=-1892..86821"),
        String::from("on x=-53470..21291,y=-120233..-33476,z=-44150..38147"),
        String::from("off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"),
    ];
    let answer = day22_part2(v.into_iter());

    assert_eq!(2758514936282235, answer);
}
