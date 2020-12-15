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

fn speak(prev: i64, turn: usize, acc: &mut HashMap<i64, Vec<usize>>) -> i64 {
    if !acc.contains_key(&prev) {
        // dbg!(turn, prev, "new");
        acc.insert(prev, vec![turn - 1]);

        0
    } else {
        let v = acc.get_mut(&prev).unwrap();
        v.push(turn - 1);

        // dbg!(turn, prev, &v);
        let tail = v.iter().rev().take(2).collect::<Vec<_>>();
        let diff = tail[0] - tail[1];
        // dbg!(turn, prev, diff);
        diff as i64
    }
}

fn generate(starting_numbers: &[i64], turns: usize) -> i64 {
    let mut acc: HashMap<i64, Vec<usize>> = HashMap::new();

    let seed_size = starting_numbers.len();

    for (turn, n) in starting_numbers[..seed_size - 1].iter().enumerate() {
        acc.insert(*n, vec![turn + 1]);
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
