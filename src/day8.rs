use std::collections::HashSet;

pub type Item = Instruction;

#[derive(Clone)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn to_nop(&self) -> Self {
        Instruction::Nop(match self {
            Self::Jmp(n) => *n,
            _ => panic!(),
        })
    }
    fn to_jmp(&self) -> Self {
        Instruction::Jmp(match self {
            Self::Nop(n) => *n,
            _ => panic!(),
        })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|s| {
            let mut pairs = s.split_whitespace();

            match pairs.next().unwrap() {
                "nop" => Instruction::Nop(pairs.next().unwrap().parse().unwrap()),
                "acc" => Instruction::Acc(pairs.next().unwrap().parse().unwrap()),
                "jmp" => Instruction::Jmp(pairs.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn compute(inst: &[Instruction]) -> isize {
    let mut seen = HashSet::new();
    let mut pos = 0_usize;
    let mut acc = 0_isize;
    loop {
        if seen.contains(&pos) {
            return acc;
        }
        seen.insert(pos);
        match &inst[pos] {
            Instruction::Acc(n) => {
                acc += n;
                pos += 1;
            }
            Instruction::Jmp(n) => {
                let n = *n;
                if n >= 0 {
                    pos += n as usize;
                } else {
                    pos -= n.abs() as usize;
                }
            }
            Instruction::Nop(_) => {
                pos += 1;
            }
        }
    }
}

fn compute2(inst: &[Instruction]) -> Result<isize, ()> {
    let mut seen = HashSet::new();
    let mut pos = 0_usize;
    let mut acc = 0_isize;
    loop {
        if seen.contains(&pos) {
            return Err(());
        }
        if inst.len() <= pos {
            return Ok(acc);
        }
        seen.insert(pos);

        match &inst[pos] {
            Instruction::Acc(n) => {
                acc += n;
                pos += 1;
            }
            Instruction::Jmp(n) => {
                let n = *n;
                if n >= 0 {
                    pos += n as usize;
                } else {
                    pos -= n.abs() as usize;
                }
            }
            Instruction::Nop(_) => {
                pos += 1;
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Item]) -> isize {
    compute(input)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Item]) -> isize {
    let nops: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|pair| match pair.1 {
            Instruction::Nop(_) => Some(pair.0),
            _ => None,
        })
        .collect();

    let jmps: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|pair| match pair.1 {
            Instruction::Jmp(_) => Some(pair.0),
            _ => None,
        })
        .collect();

    for idx in nops {
        let mut input = input.to_vec();
        input[idx] = input[idx].to_jmp();
        if let Ok(acc) = compute2(&input) {
            return acc;
        }
    }
    for idx in jmps {
        let mut input = input.to_vec();
        input[idx] = input[idx].to_nop();
        if let Ok(acc) = compute2(&input) {
            return acc;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const INPUT: &str = r##"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"##;

    #[test]
    fn test_something() {
        assert_eq!(5, solve_part1(&input_generator(INPUT)));
    }
    #[test]
    fn test_something2() {
        // assert_eq!(8, solve_part2(&input_generator(INPUT)));
    }
}
