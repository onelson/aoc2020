use std::collections::HashMap;
use std::fmt;

pub type Item = i32;

#[derive(Debug)]
pub struct Pair(usize, usize);

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Item> {
    let mut xs = input
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect::<Vec<Item>>();
    xs.sort();
    xs
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Item]) -> usize {
    let pair = calculate(input);
    pair.0 * pair.1
}

fn calculate(input: &[i32]) -> Pair {
    let mut joltage = 0;
    let mut ones = 0;
    let mut threes = 0;

    for i in input {
        match i - joltage {
            1 => ones += 1,
            3 => threes += 1,
            _ => continue,
        }
        joltage = *i;
    }

    threes += 1; // built-in

    println!("{} + {} = {}", ones, threes, ones + threes);
    Pair(ones, threes)
}

fn count(input: &[Item]) -> usize {
    let mut acc: HashMap<i32, i64> = HashMap::new();
    acc.insert(0, 1);
    for n in input {
        for prev in 1..=3 {
            if let Some(val) = acc.get(&(*n - prev)).copied() {
                let entry = acc.entry(*n).or_insert(0);
                *entry += val;
            }
        }
    }
    *acc.get(&(*input.iter().max().unwrap())).unwrap() as usize
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Item]) -> usize {
    count(input)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    #[test]
    fn test_smaller() {
        let pair = calculate(&input_generator(SMALL));
        assert_eq!(7, pair.0);
        assert_eq!(5, pair.1);
    }

    #[test]
    fn test_larger() {
        let pair = calculate(&input_generator(LARGER));
        assert_eq!(22, pair.0);
        assert_eq!(10, pair.1);
    }

    #[test]
    fn test_count_smaller() {
        assert_eq!(8, count(&input_generator(SMALL)));
    }
    #[test]
    fn test_count_larger() {
        assert_eq!(19208, count(&input_generator(LARGER)))
    }

    const SMALL: &str = r##"16
10
15
5
1
11
7
19
6
12
4"##;
    const LARGER: &str = r##"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"##;
}
