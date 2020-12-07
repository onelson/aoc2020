use std::collections::{HashMap, HashSet};

pub type Item = HashMap<String, Vec<(usize, String)>>;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Item {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (key, tail) = {
            let mut it = line.splitn(2, " bags contain ");
            (it.next().unwrap(), it.next().unwrap())
        };
        let children: Vec<(usize, String)> = if tail.contains("no other bags") {
            vec![]
        } else {
            tail.split(",")
                .map(|s| {
                    let mut parts = s.split_whitespace();
                    let count = parts.next().unwrap().parse::<usize>().unwrap();
                    let x: String = parts
                        .take_while(|s| {
                            let s = s.trim_end_matches(".");
                            !(s.ends_with("bag") || s.ends_with("bags"))
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    (count, x)
                })
                .collect()
        };

        map.insert(key.to_string(), children);
    }
    map
}

fn can_contain(target: &String, input: &Item) -> HashSet<String> {
    input
        .iter()
        .filter_map(|(k, v)| {
            if v.iter().any(|(_n, color)| color == target) {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect()
}

fn trace_parents(target: &String, input: &Item) -> HashSet<String> {
    can_contain(target, input)
        .into_iter()
        .map(|x| {
            let mut xs = trace_parents(&x, input);
            xs.insert(x);
            xs
        })
        .fold(HashSet::new(), |acc, xs| acc.union(&xs).cloned().collect())
}

fn trace_children(count: usize, target: &String, input: &Item) -> usize {
    let children: HashSet<(usize, String)> = input
        .get(target)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    children
        .into_iter()
        .fold(0, |acc, (n, x)| acc + trace_children(count * n, &x, input))
        + count
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Item) -> usize {
    let target = "shiny gold".to_string();
    let x = trace_parents(&target, input);
    x.len()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Item) -> usize {
    let target = "shiny gold".to_string();
    trace_children(1, &target, input) - 1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const INPUT: &str = r##"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."##;

    const INPUT2: &str = r##"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."##;

    #[test]
    fn test_something() {
        assert_eq!(4, solve_part1(&input_generator(INPUT)));
    }
    #[test]
    fn test_something2() {
        assert_eq!(126, solve_part2(&input_generator(INPUT2)));
    }
}
