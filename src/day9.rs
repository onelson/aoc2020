use std::collections::VecDeque;

pub type Item = usize;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Item]) -> usize {
    validate(25, input.to_vec()).unwrap()
}

fn validate(preamble_size: usize, xs: Vec<Item>) -> Option<usize> {
    let mut buf = VecDeque::with_capacity(5);
    let mut it = xs.into_iter();
    for _ in 0..preamble_size {
        buf.push_back(it.next().unwrap());
    }

    for i in it {
        if !check_sums(i, &buf) {
            return Some(i);
        }
        buf.pop_front();
        buf.push_back(i);
    }
    None
}

fn check_sums(target: usize, buf: &VecDeque<usize>) -> bool {
    for (i, x) in buf.iter().enumerate() {
        for (j, y) in buf.iter().enumerate() {
            if i == j {
                continue;
            }
            if x + y == target {
                return true;
            }
        }
    }
    false
}

fn find_weakness(target: usize, pos: usize, buf: &[usize]) -> usize {
    for idx in pos..buf.len() {
        let sum: usize = buf[pos..idx].iter().sum();
        if sum == target {
            return buf[pos..idx].iter().min().unwrap() + buf[pos..idx].iter().max().unwrap();
        } else if sum > target {
            return find_weakness(target, pos + 1, buf);
        }
    }
    unreachable!()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Item]) -> usize {
    let xs = input.to_vec();
    let target = validate(25, xs).unwrap();
    find_weakness(target, 0, input)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    const INPUT: &str = r##"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"##;

    #[test]
    fn test_something() {
        assert_eq!(127, validate(5, input_generator(INPUT)).unwrap());
    }

    #[test]
    fn test_something2() {
        let input = input_generator(INPUT);
        assert_eq!(127, validate(5, input.to_vec()).unwrap());
        assert_eq!(62, find_weakness(127, 0, &input));
    }
}
