use std::collections::HashSet;

pub type Item = Vec<Vec<char>>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Item]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut acc = HashSet::<char>::new();
            for person in group {
                acc.extend(person);
            }
            acc.len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Item]) -> usize {
    input
        .into_iter()
        .map(|group| {
            let sets = group
                .into_iter()
                .map(|x| x.into_iter().cloned().collect::<HashSet<char>>())
                .collect::<Vec<_>>();
            let mut it = sets.into_iter();
            let first = HashSet::<char>::from(it.next().unwrap());
            it.fold(first, |acc, x| acc.intersection(&x).cloned().collect())
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn test_something() {
        assert_eq!(11, solve_part1(&input_generator(INPUT)));
    }

    #[test]
    fn test_something2() {
        assert_eq!(6, solve_part2(&input_generator(INPUT)));
    }
}
