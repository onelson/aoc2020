type Item = Vec<char>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input.lines().map(|s| s.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Item]) -> usize {
    slope(3, 1, input)
}

fn slope(r: usize, d: usize, input: &[Item]) -> usize {
    let mut count = 0;
    let mut rows = input.iter().step_by(d);
    rows.next(); // drop initial row(s)
    let mut i = r;
    for row in rows {
        if &'#' == row.iter().cycle().nth(i).unwrap() {
            count += 1;
        }
        i += r;
    }
    count
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Item]) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|(r, d)| slope(*r, *d, input)).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"##;

    #[test]
    fn test_part1() {
        assert_eq!(7, slope(3, 1, &input_generator(INPUT)));
    }

    #[test]
    fn test_part2_case1() {
        assert_eq!(2, slope(1, 1, &input_generator(INPUT)));
    }
    #[test]
    fn test_part2_case3() {
        assert_eq!(3, slope(5, 1, &input_generator(INPUT)));
    }

    #[test]
    fn test_part2_case4() {
        assert_eq!(4, slope(7, 1, &input_generator(INPUT)));
    }

    #[test]
    fn test_part2_case5() {
        assert_eq!(2, slope(1, 2, &input_generator(INPUT)));
    }
}
