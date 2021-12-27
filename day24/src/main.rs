use std::{collections::HashMap, env, io, io::prelude::*};

use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{:?}", day24(values)),
        _ => println!("Invalid part {}", part),
    }
}

const W: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;

#[derive(Debug, Clone, Copy)]
enum Op {
    Reg(usize),
    Val(i64),
}

fn to_reg(s: &str) -> usize {
    match s {
        "w" => W,
        "x" => X,
        "y" => Y,
        "z" => Z,
        _ => panic!("Invalid register {}", s),
    }
}

impl Op {
    fn from(s: &str) -> Op {
        if let Ok(n) = s.parse::<i64>() {
            Op::Val(n)
        } else {
            Op::Reg(to_reg(s))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(usize),
    Add(usize, Op),
    Mul(usize, Op),
    Div(usize, Op),
    Mod(usize, Op),
    Eql(usize, Op),
}

impl Instruction {
    fn from(str: &str) -> Instruction {
        let mut parts = str.split_whitespace();
        let inst = parts.next().unwrap();
        let a = to_reg(parts.next().unwrap());
        let b = parts.next();
        match inst {
            "inp" => Instruction::Inp(a),
            "add" => Instruction::Add(a, Op::from(b.unwrap())),
            "mul" => Instruction::Mul(a, Op::from(b.unwrap())),
            "div" => Instruction::Div(a, Op::from(b.unwrap())),
            "mod" => Instruction::Mod(a, Op::from(b.unwrap())),
            "eql" => Instruction::Eql(a, Op::from(b.unwrap())),
            _ => panic!("Invalid op {}", inst),
        }
    }

    fn op(op: &Op, reg: &[i64; 4]) -> i64 {
        match op {
            Op::Val(n) => *n,
            Op::Reg(r) => reg[*r],
        }
    }

    fn exec(&self, reg: &mut [i64; 4], input: Option<i64>) {
        match self {
            Instruction::Inp(a) => reg[*a] = input.unwrap(),
            Instruction::Add(a, b) => reg[*a] += Instruction::op(b, reg),
            Instruction::Mul(a, b) => reg[*a] *= Instruction::op(b, reg),
            Instruction::Div(a, b) => {
                let b = Instruction::op(b, reg);
                if b == 0 {
                    panic!("Division by zero");
                }
                reg[*a] /= b;
            }
            Instruction::Mod(a, b) => {
                let b = Instruction::op(b, reg);
                if b == 0 {
                    panic!("Division by zero");
                }
                reg[*a] %= b;
            }
            Instruction::Eql(a, b) => {
                let b = Instruction::op(b, reg);
                reg[*a] = if reg[*a] == b { 1 } else { 0 };
            }
        }
    }
}

struct MonadCPU<'a, const LEN: usize> {
    cache: HashMap<(usize, i64), Option<i64>>,
    digit_inst: Vec<&'a [Instruction]>,
    range: [i64; 9],
}

impl<'a, const LEN: usize> MonadCPU<'a, LEN> {
    fn new(instructions: &'a Vec<Instruction>, range: [i64; 9]) -> MonadCPU<'a, LEN> {
        let digit_inst = instructions
            .chunks(instructions.len() / LEN)
            .collect::<Vec<_>>();

        assert_eq!(digit_inst.len(), LEN);

        MonadCPU {
            cache: HashMap::new(),
            digit_inst,
            range,
        }
    }

    fn execute(&mut self, position: usize, reg: [i64; 4]) -> Option<i64> {
        if let Some(&best_digit) = self.cache.get(&(position, reg[Z])) {
            return best_digit;
        }

        // Search for the first digit (9 through 1, or 1 through 9) that produces a target Z
        // value (initially 0) for the current instruction block. Each block of instructions maps
        // to a position in the resulting number, starting from the left.
        for digit in self.range {
            let mut reg = reg;

            for &instruction in self.digit_inst[position] {
                instruction.exec(&mut reg, Some(digit));
            }

            // The Z register values for each successive digit will be consistent on their
            // way to producing 0 for the last digit.
            let z = reg[Z];

            if position < LEN - 1 {
                // Search the next digit in the next instruction block
                if let Some(best_digit) = self.execute(position + 1, reg) {
                    self.cache.insert((position, z), Some(best_digit));

                    // Construct the resulting number (which is actually in reverse, but we
                    // take care of that later)
                    return Some(best_digit * 10 + digit);
                }
            } else if z == 0 {
                // Last digit, and we found a solution
                self.cache.insert((position, 0), Some(digit));
                return Some(digit);
            }
        }

        // No solution found
        self.cache.insert((position, reg[Z]), None);
        None
    }
}

fn reverse_number(n: i64) -> i64 {
    let mut n = n;
    let mut result = 0;

    while n > 0 {
        result *= 10;
        result += n % 10;
        n /= 10;
    }

    result
}

fn day24(v: impl Iterator<Item = String>) -> [i64; 2] {
    let instructions = v.map(|x| Instruction::from(&x)).collect::<Vec<_>>();
    let mut cpus = [
        MonadCPU::<14>::new(&instructions, [9, 8, 7, 6, 5, 4, 3, 2, 1]),
        MonadCPU::<14>::new(&instructions, [1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ];

    let vals = cpus
        .par_iter_mut()
        .map(|cpu| reverse_number(cpu.execute(0, [0, 0, 0, 0]).unwrap()))
        .collect::<Vec<_>>();

    [vals[0], vals[1]]
}
