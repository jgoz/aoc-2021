use core::fmt;
use pathfinding::directed::astar::astar;
use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::{Debug, Display, Formatter},
    io,
    io::prelude::*,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day23_part1(values)),
        "2" => println!("{}", day23_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Species {
    A = 1,
    B = 10,
    C = 100,
    D = 1000,
}

impl Species {
    fn from(char: char) -> Option<Species> {
        match char {
            'A' => Some(Self::A),
            'B' => Some(Self::B),
            'C' => Some(Self::C),
            'D' => Some(Self::D),
            _ => None,
        }
    }

    fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::A),
            1 => Some(Self::B),
            2 => Some(Self::C),
            3 => Some(Self::D),
            _ => None,
        }
    }

    fn cost(&self) -> u32 {
        *self as u32
    }

    fn room_index(&self) -> i32 {
        match self {
            &Self::A => 2,
            &Self::B => 4,
            &Self::C => 6,
            &Self::D => 8,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Amphipod {
    species: Species,
    id: usize,
}

impl Amphipod {
    fn from(char: char, id: usize) -> Option<Amphipod> {
        if let Some(species) = Species::from(char) {
            Some(Amphipod { species, id })
        } else {
            None
        }
    }
}

type Loc = (i32, i32);

fn distance(a: &Loc, b: &Loc, cost: u32) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32 * cost
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Space {
    Hallway,
    Doorway,
    Room(Species),
}

#[derive(Clone, PartialEq, Eq)]
struct Board<const SIZE: usize> {
    id: usize,
    spaces: HashMap<Loc, Space>,
    amphipods: HashMap<Amphipod, Loc>,
}

impl<const SIZE: usize> Board<SIZE> {
    fn from(v: impl Iterator<Item = String>) -> Board<SIZE> {
        let mut board = Board {
            id: 0,
            spaces: HashMap::new(),
            amphipods: HashMap::new(),
        };

        let mut id = 0;
        for (y, line) in v.skip(1).enumerate() {
            for (x, char) in line.chars().skip(1).enumerate() {
                if let Some(amph) = Amphipod::from(char, id) {
                    board.amphipods.insert(amph, (x as i32, y as i32));
                    id += 1;
                }
            }
        }

        for i in 0..4 {
            let species = Species::from_index(i).unwrap();
            let x = species.room_index();
            for y in 1..=SIZE {
                board.spaces.insert((x, y as i32), Space::Room(species));
            }
        }

        board.spaces.insert((0, 0), Space::Hallway);
        board.spaces.insert((1, 0), Space::Hallway);
        board.spaces.insert((2, 0), Space::Doorway);
        board.spaces.insert((3, 0), Space::Hallway);
        board.spaces.insert((4, 0), Space::Doorway);
        board.spaces.insert((5, 0), Space::Hallway);
        board.spaces.insert((6, 0), Space::Doorway);
        board.spaces.insert((7, 0), Space::Hallway);
        board.spaces.insert((8, 0), Space::Doorway);
        board.spaces.insert((9, 0), Space::Hallway);
        board.spaces.insert((10, 0), Space::Hallway);

        board
    }

    fn to_string(&self) -> String {
        #[rustfmt::skip]
        let mut lines  = vec![
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            [' ', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ', ' '],
        ];

        for _ in 1..SIZE {
            lines.insert(
                3,
                [
                    ' ', ' ', '#', '.', '#', '.', '#', '.', '#', '.', '#', ' ', ' ',
                ],
            );
        }

        for (amp, loc) in &self.amphipods {
            lines[loc.1 as usize + 1][loc.0 as usize + 1] = match amp.species {
                Species::A => 'A',
                Species::B => 'B',
                Species::C => 'C',
                Species::D => 'D',
            };
        }
        lines
            .into_iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn is_occupied(&self, loc: &Loc) -> bool {
        !self.spaces.contains_key(loc) || self.amphipods.values().any(|amp| amp == loc)
    }

    fn last_empty_room_slot(&self, species: Species) -> Option<Loc> {
        let x = species.room_index();
        for y in (1..=SIZE).rev() {
            let loc = (x, y as i32);
            if !self.is_occupied(&loc) {
                return Some(loc);
            }
        }
        None
    }

    fn room_is_ok(&self, species: Species) -> bool {
        let x = species.room_index();
        for y in 1..=SIZE {
            if let Some(amph) = self.amphipods.iter().find(|a| a.1 == &(x, y as i32)) {
                if amph.0.species != species {
                    return false;
                }
            }
        }
        true
    }

    fn is_finished(&self) -> bool {
        self.amphipods
            .iter()
            .all(|(amp, loc)| self.spaces.get(loc) == Some(&Space::Room(amp.species)))
    }

    fn successors(&self, &(x, y): &Loc, cost: u32) -> Vec<(Loc, u32)> {
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|p| !self.is_occupied(p))
            .map(|p| (p, cost))
            .collect()
    }

    fn possible_moves(&self) -> Vec<(Amphipod, Loc, u32)> {
        let mut room_moves = vec![];
        let mut hall_moves = vec![];

        for (amph, start) in self.amphipods.iter() {
            let &cur_space = self.spaces.get(start).unwrap();
            if cur_space == Space::Room(amph.species) {
                if self.room_is_ok(amph.species) {
                    continue;
                }
            }

            for (dest, &space) in self.spaces.iter() {
                if dest == start || self.is_occupied(&dest) {
                    continue;
                }
                match space {
                    Space::Doorway => (),
                    Space::Hallway => {
                        if cur_space != Space::Hallway {
                            // They never move from a hallway to a hallway
                            let result = astar(
                                start,
                                |p| self.successors(p, amph.species.cost()),
                                |p| distance(p, &dest, amph.species.cost()),
                                |p| p == dest,
                            );
                            if let Some((_, cost)) = result {
                                hall_moves.push((*amph, *dest, cost));
                            }
                        }
                    }
                    Space::Room(species) => {
                        // Only walk into a room if it's the same species
                        if species == amph.species
                            && self.room_is_ok(species)
                            && self.last_empty_room_slot(species).unwrap() == *dest
                        {
                            let result = astar(
                                start,
                                |p| self.successors(p, amph.species.cost()),
                                |p| distance(p, &dest, amph.species.cost()),
                                |p| p == dest,
                            );
                            if let Some((_, cost)) = result {
                                // Short circuit if we can move directly into a destination room
                                room_moves.push((*amph, *dest, cost));
                            }
                        }
                    }
                };
            }
        }

        room_moves.sort_by_key(|m| m.2);
        hall_moves.sort_by_key(|m| m.2);

        room_moves.append(&mut hall_moves);
        room_moves
    }

    fn move_amphipod(&self, amph: &Amphipod, loc: Loc) -> Board<SIZE> {
        let mut new_board = self.clone();
        new_board.id = self.id + 1;
        new_board.amphipods.insert(*amph, loc);
        new_board
    }

    fn solve(&self) -> u32 {
        let mut queue: Vec<(Board<SIZE>, u32)> = vec![(self.clone(), 0)];

        let mut min_cost = u32::max_value();
        let mut tried = HashSet::new();

        while queue.len() > 0 {
            let (board, head_cost) = queue.remove(0);

            let moves = board.possible_moves();
            for (amph, loc, cost) in moves {
                let new_cost = head_cost + cost;
                if new_cost > min_cost {
                    continue;
                }
                let new_board = board.move_amphipod(&amph, loc);
                let hash = new_board.to_string();
                let is_finished = new_board.is_finished();
                if is_finished {
                    println!("SOLUTION {}\n{:?}\n", new_cost, new_board);
                    if new_cost < min_cost {
                        min_cost = new_cost;
                    }
                    continue;
                }
                if tried.insert(hash) {
                    queue.push((new_board, new_cost));
                }
            }
        }

        min_cost
    }
}

impl<const SIZE: usize> Debug for Board<SIZE> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl<const SIZE: usize> Display for Board<SIZE> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn day23_part1(v: impl Iterator<Item = String>) -> u32 {
    let board = Board::<2>::from(v);

    println!("{:?}", board);

    let cost = board.solve();

    if cost == u32::MAX {
        panic!("Couldn't solve");
    }

    cost
}

#[test]
fn day23_part1_test() {
    let v = vec![
        String::from("#############"),
        String::from("#...........#"),
        String::from("###B#C#B#D###"),
        String::from("  #A#D#C#A#"),
        String::from("  #########"),
    ];

    let answer = day23_part1(v.into_iter());

    assert_eq!(Species::A.cost(), 1);
    assert_eq!(Species::B.cost(), 10);
    assert_eq!(Species::C.cost(), 100);
    assert_eq!(Species::D.cost(), 1000);

    assert_eq!(12521, answer);
}

#[test]
fn day23_part1_test_3() {
    let v = vec![
        String::from("#############"),
        String::from("#...B.....A.#"),
        String::from("###A#D#C#.###"),
        String::from("  #C#D#B#.#"),
        String::from("  #########"),
    ];

    let test_board = Board::<2>::from(v.clone().into_iter());
    println!("{:?}", test_board);
    let possibles = test_board.possible_moves();

    for possible in possibles {
        let new_board = test_board.move_amphipod(&possible.0, possible.1);
        println!("{:?}", possible.2);
        println!("{:?}", new_board);
    }
}

fn day23_part2(v: impl Iterator<Item = String>) -> u32 {
    let mut big_v = v.collect::<Vec<_>>();
    big_v.insert(3, "  #D#C#B#A#".to_string());
    big_v.insert(4, "  #D#B#A#C#".to_string());

    let board = Board::<4>::from(big_v.into_iter());

    println!("{:?}", board);

    let cost = board.solve();

    if cost == u32::MAX {
        panic!("Couldn't solve");
    }

    cost
}

#[test]
fn day23_part2_test() {
    let v = vec![
        String::from("#############"),
        String::from("#...........#"),
        String::from("###B#C#B#D###"),
        String::from("  #A#D#C#A#"),
        String::from("  #########"),
    ];

    let answer = day23_part2(v.into_iter());

    assert_eq!(44169, answer);
}
