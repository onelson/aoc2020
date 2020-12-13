#[derive(Debug)]
pub struct Notes {
    earliest_departure: u64,
    bus_ids: Vec<Option<u64>>,
}

pub type Item = Notes;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Notes {
    let mut it = input.lines();
    Notes {
        earliest_departure: it.next().unwrap().parse().unwrap(),
        bus_ids: it
            .next()
            .unwrap()
            .split(',')
            .map(|x| match x {
                "x" => None,
                other => Some(other.parse().unwrap()),
            })
            .collect(),
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(
    Notes {
        earliest_departure,
        bus_ids,
    }: &Item,
) -> u64 {
    let best_fit = bus_ids
        .iter()
        .filter_map(|&x| x)
        .map(|period| {
            let x = earliest_departure / period;
            if earliest_departure % period == 0 {
                (period, *earliest_departure)
            } else {
                (period, (x * period) + period)
            }
        })
        .min_by_key(|pair| pair.1)
        .unwrap();
    best_fit.0 * (best_fit.1 - earliest_departure)
}

#[aoc(day13, part2)]
pub fn solve_part2(Notes { bus_ids, .. }: &Item) -> usize {
    let bus_ids: Vec<(usize, u64)> = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(idx, bus_id)| bus_id.map(|id| (idx, id)))
        .collect();

    let mut t = 0;
    let mut step = 1;
    for (offset, bus_id) in bus_ids {
        loop {
            if (t + offset) % bus_id as usize == 0 {
                break;
            }
            t += step;
        }
        dbg!(step);
        // Multiply the step with each bus match found.
        // Seems to work, but I don't know why.
        step *= bus_id as usize;
    }
    dbg!(t)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    const INPUT: &str = r##"939
7,13,x,x,59,x,31,19"##;

    #[test]
    fn test_something() {
        assert_eq!(295, solve_part1(&input_generator(INPUT)));
    }
    #[test]
    fn test_something2() {
        assert_eq!(1068781, solve_part2(&input_generator(INPUT)));
    }
}
