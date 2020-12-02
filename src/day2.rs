use std::str::FromStr;

pub struct Entry {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let range = parts.next().unwrap();
        let char = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap().to_string();

        let mut numbers = range.splitn(2, "-");
        let min = numbers.next().unwrap().parse().unwrap();
        let max = numbers.next().unwrap().parse().unwrap();

        Ok(Entry {
            min,
            max,
            char,
            password,
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Entry]) -> usize {
    input
        .into_iter()
        .filter(|x| {
            let count = x.password.chars().filter(|c| c == &x.char).count();
            count >= x.min && count <= x.max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Entry]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};
    use crate::day2::Entry;

    #[test]
    fn test_part1_case1() {
        let entry: Entry = "1-3 a: abcde".parse().unwrap();
        assert!(entry.is_valid());
    }
    #[test]
    fn test_part1_case2() {
        let entry: Entry = "1-3 b: cdefg".parse().unwrap();
        assert!(!entry.is_valid());
    }
    #[test]
    fn test_part1_case3() {
        let entry: Entry = "2-9 c: ccccccccc".parse().unwrap();
        assert!(entry.is_valid());
    }
}
