use std::collections::HashMap;

pub type Item = i64;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Item]) -> i64 {
    generate(input, 2020)
}

#[derive(Debug)]
struct History(Option<usize>, Option<usize>);

impl History {
    fn new(turn: usize) -> Self {
        Self(Some(turn), None)
    }
    fn push(&mut self, turn: usize) {
        self.1 = self.0.take();
        self.0 = Some(turn);
    }
    fn delta(&self) -> i64 {
        self.0.unwrap() as i64 - self.1.unwrap() as i64
    }
}

fn speak(prev: i64, turn: usize, acc: &mut HashMap<i64, History>) -> i64 {
    if !acc.contains_key(&prev) {
        // dbg!(turn, prev, "new");
        acc.insert(prev, History::new(turn - 1));
        0
    } else {
        let hist = acc.get_mut(&prev).unwrap();
        hist.push(turn - 1);

        // dbg!(turn, prev, &v);
        // dbg!(turn, prev, diff);
        hist.delta()
    }
}

fn generate(starting_numbers: &[i64], turns: usize) -> i64 {
    let mut acc: HashMap<i64, History> = HashMap::new();

    let seed_size = starting_numbers.len();

    for (turn, n) in starting_numbers[..seed_size - 1].iter().enumerate() {
        acc.insert(*n, History::new(turn + 1));
    }

    let mut prev = starting_numbers[seed_size - 1];
    for turn in (seed_size + 1)..=turns {
        prev = speak(prev, turn, &mut acc);
    }

    prev
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Item]) -> i64 {
    generate(input, 30000000)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(0, generate(&[0, 3, 6], 10));
        assert_eq!(1, generate(&[1, 3, 2], 2020));
        assert_eq!(10, generate(&[2, 1, 3], 2020));
        assert_eq!(27, generate(&[1, 2, 3], 2020));
        assert_eq!(78, generate(&[2, 3, 1], 2020));
        assert_eq!(438, generate(&[3, 2, 1], 2020));
        assert_eq!(1836, generate(&[3, 1, 2], 2020));
    }
}
