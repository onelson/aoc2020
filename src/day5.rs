pub type Item = (Vec<Row>, Vec<Col>);

pub enum Row {
    Upper,
    Lower,
}

pub enum Col {
    Upper,
    Lower,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|line| {
            let (rows, cols) = line.split_at(7);
            (
                rows.chars()
                    .map(|c| match c {
                        'F' => Row::Lower,
                        'B' => Row::Upper,
                        _ => unreachable!(),
                    })
                    .collect(),
                cols.chars()
                    .map(|c| match c {
                        'L' => Col::Lower,
                        'R' => Col::Upper,
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        })
        .collect()
}

fn get_seat_ids(input: &[Item]) -> Vec<usize> {
    input
        .iter()
        .map(|(rs, cs)| {
            let all_rows: Vec<usize> = (0..=127).collect();
            let all_cols: Vec<usize> = (0..=7).collect();

            let mut rows = &all_rows[..];
            let mut cols = &all_cols[..];

            for r in rs {
                let mid = (rows.len() + 1) / 2;

                match r {
                    Row::Upper => {
                        rows = &rows[mid..];
                    }
                    Row::Lower => {
                        rows = &rows[..mid];
                    }
                }
            }

            for c in cs {
                let mid = (cols.len() + 1) / 2;

                match c {
                    Col::Upper => {
                        cols = &cols[mid..];
                    }
                    Col::Lower => {
                        cols = &cols[..mid];
                    }
                }
            }

            seat_id(rows[0], cols[0])
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Item]) -> usize {
    get_seat_ids(input).into_iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Item]) -> usize {
    let mut ids = input
        .iter()
        .map(|(rs, cs)| {
            let all_rows: Vec<usize> = (0..=127).collect();
            let all_cols: Vec<usize> = (0..=7).collect();

            let mut rows = &all_rows[..];
            let mut cols = &all_cols[..];

            for r in rs {
                let mid = (rows.len() + 1) / 2;

                match r {
                    Row::Upper => {
                        rows = &rows[mid..];
                    }
                    Row::Lower => {
                        rows = &rows[..mid];
                    }
                }
            }

            for c in cs {
                let mid = (cols.len() + 1) / 2;
                match c {
                    Col::Upper => {
                        cols = &cols[mid..];
                    }
                    Col::Lower => {
                        cols = &cols[..mid];
                    }
                }
            }

            seat_id(rows[0], cols[0])
        })
        .collect::<Vec<usize>>();

    ids.sort();

    let mut prev: Option<usize> = None;
    for id in ids {
        if let Some(prev) = prev {
            if id - prev > 1 {
                return id - 1;
            }
        }
        prev = Some(id)
    }
    unreachable!()
}

fn seat_id(row: usize, col: usize) -> usize {
    row * 8 + col
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"##;

    #[test]
    fn test1() {
        assert_eq!(820, solve_part1(&input_generator(INPUT)));
    }

    // #[test]
    // fn test2() {
    //     assert_eq!(11, solve_part2(&input_generator(INPUT)));
    // }
}
