#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    for x in input {
        for y in input {
            if x == y {
                continue;
            }
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    for x in input {
        for y in input {
            for z in input {
                if x == y || x == z || y == z {
                    continue;
                }
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::day1::{solve_part1, solve_part2};

    #[test]
    fn test_part1_case1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, solve_part1(&input));
    }

    #[test]
    fn test_part1_case2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, solve_part2(&input));
    }
}
