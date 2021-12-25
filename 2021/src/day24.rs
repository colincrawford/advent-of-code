use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Val {
    Register(usize),
    Value(i64),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Inp(usize),
    Add(usize, Val),
    Mul(usize, Val),
    Div(usize, Val),
    Mod(usize, Val),
    Eql(usize, Val),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Alu {
    mem: [i64; 4],
}

impl Alu {
    fn run(&mut self, op: Op) {
        match op {
            Op::Inp(_) => panic!("Cannot apply Inp instruction"),
            Op::Add(r, v) => self.mem[r] += self.get_val(v),
            Op::Mul(r, v) => self.mem[r] *= self.get_val(v),
            Op::Div(r, v) => self.mem[r] /= self.get_val(v),
            Op::Mod(r, v) => self.mem[r] %= self.get_val(v),
            Op::Eql(r, v) => self.mem[r] = (self.mem[r] == self.get_val(v)) as i64,
        }
    }

    fn apply_inp(&mut self, register_inx: usize, val: i64) {
        self.mem[register_inx] = val;
    }

    fn get_val(&self, v: Val) -> i64 {
        match v {
            Val::Register(i) => self.mem[i],
            Val::Value(f) => f,
        }
    }
}

fn solve(ops: &Vec<Op>) -> (u64, u64) {
    let mut alus: Vec<(Alu, (u64, u64))> = vec![(Alu { mem: [0; 4] }, (0, 0))];
    for op in ops {
        match op {
            Op::Inp(t) => {
                let mut new_alus: Vec<(Alu, (u64, u64))> = Vec::new();
                let mut indizes: HashMap<Alu, usize> = HashMap::new();
                for alu in &alus {
                    for v in 1..=9 {
                        let mut new_alu = alu.clone();
                        new_alu.0.apply_inp(*t, v);
                        new_alu.1 .0 = new_alu.1 .0 * 10 + v as u64;
                        new_alu.1 .1 = new_alu.1 .1 * 10 + v as u64;
                        if let Some(idx) = indizes.get(&new_alu.0) {
                            new_alus[*idx].1 .0 = u64::min(new_alus[*idx].1 .0, new_alu.1 .0);
                            new_alus[*idx].1 .1 = u64::max(new_alus[*idx].1 .1, new_alu.1 .1);
                        } else {
                            indizes.insert(new_alu.0.clone(), new_alus.len());
                            new_alus.push(new_alu);
                        }
                    }
                }
                alus = new_alus;
            }
            op => {
                for alu in &mut alus {
                    alu.0.run(*op);
                }
            }
        }
    }
    let alus_: Vec<_> = alus.iter().filter(|alu| alu.0.mem[3] == 0).collect();
    let lowest = alus_.iter().map(|alu| alu.1 .0);
    let highest = alus_.iter().map(|alu| alu.1 .1);
    (lowest.min().unwrap(), highest.max().unwrap())
}

fn register_to_mem_index(s: &str) -> Option<usize> {
    match s {
        "w" => Some(0),
        "x" => Some(1),
        "y" => Some(2),
        "z" => Some(3),
        _ => None,
    }
}

fn parse_line(line: &str) -> Op {
    let op_pieces: Vec<_> = line.split(' ').collect();
    let register = register_to_mem_index(op_pieces[1]).unwrap();
    let val = if op_pieces.len() < 3 {
        Val::Value(0)
    } else if let Some(idx) = register_to_mem_index(op_pieces[2]) {
        Val::Register(idx)
    } else {
        Val::Value(op_pieces[2].parse::<i64>().unwrap())
    };
    match op_pieces[0] {
        "inp" => Op::Inp(register),
        "add" => Op::Add(register, val),
        "mul" => Op::Mul(register, val),
        "div" => Op::Div(register, val),
        "mod" => Op::Mod(register, val),
        "eql" => Op::Eql(register, val),
        _ => panic!("Invalid instruction {}", op_pieces[0]),
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input.lines().map(|l| parse_line(l)).collect()
}

pub fn day24_part1(input: &str) -> String {
    let ops = parse_input(input);
    let (lowest, highest) = solve(&ops);
    format!("highest: {}, lowest: {}", highest, lowest)
}

pub fn day24_part2(_input: &str) -> String {
    format!("see part one ^ for lowest")
}
