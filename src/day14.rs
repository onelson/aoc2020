use std::collections::HashMap;

#[derive(Debug)]
pub enum Item {
    Mask(Vec<Option<bool>>),
    Assignment { addr: usize, value: i64 },
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|s| {
            if s.starts_with("mask") {
                let mask = s
                    .replace("mask = ", "")
                    .chars()
                    .map(|c| match c {
                        'X' => None,
                        '1' => Some(true),
                        '0' => Some(false),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();
                debug_assert_eq!(36, mask.len());
                Item::Mask(mask)
            } else {
                let s = s.replace("mem[", "");
                let parts = s.splitn(2, "] = ").collect::<Vec<&str>>();
                Item::Assignment {
                    addr: parts[0].parse().unwrap(),
                    value: parts[1].parse().unwrap(),
                }
            }
        })
        .collect()
}

fn apply_mask(value: i64, mask: &[Option<bool>]) -> i64 {
    let mut value = value;
    for (pos, set_bit) in mask
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(pos, m)| m.map(|x| (pos, x)))
    {
        if set_bit {
            value |= 1 << pos;
        } else {
            value &= !(1 << pos);
        }
    }
    value
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Item]) -> i64 {
    let mut memory = HashMap::<usize, i64>::new();
    let mut mask = match &input[0] {
        Item::Mask(bits) => bits,
        _ => unreachable!(),
    };

    for item in &input[1..] {
        match item {
            Item::Mask(bits) => {
                mask = bits;
            }
            Item::Assignment { addr, value } => {
                memory.insert(*addr, apply_mask(*value, &mask));
            }
        }
    }
    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(_input: &[Item]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    const INPUT: &str = r##"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"##;

    #[test]
    fn test_something() {
        assert_eq!(165, solve_part1(&input_generator(INPUT)));
    }
}
