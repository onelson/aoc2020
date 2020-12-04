type Item = Vec<(String, String)>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Item> {
    let entries: Vec<_> = input.split("\n\n").collect();
    println!("{}", entries.len());

    entries
        .iter()
        .map(|s| {
            let keys = s
                .split_whitespace()
                .map(|s| {
                    let mut splits = s.splitn(2, ':');
                    (
                        splits.next().unwrap().to_string(),
                        splits.next().unwrap().to_string(),
                    )
                })
                .collect::<Vec<_>>();
            keys
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Item]) -> usize {
    input
        .iter()
        .filter(|xs| {
            let x = REQUIRED
                .iter()
                .all(|k| xs.iter().find(|(a, b)| &a.as_str() == k).is_some());
            println!("{:?}={}", &xs, x);
            x
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Item]) -> usize {
    input
        .iter()
        .filter(|xs| {
            let x = REQUIRED
                .iter()
                .all(|k| xs.iter().find(|(a, b)| &a.as_str() == k).is_some());
            println!("{:?}={}", &xs, x);
            x
        })
        .filter(|xs| xs.iter().all(|(a, b)| validate(a, b)))
        .count()
}

const OPTIONAL: [&str; 1] = ["cid"];
const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn validate(k: &str, v: &str) -> bool {
    let n = v.parse::<i32>();
    match k {
        "byr" => n.map(|n| n >= 1920 && n <= 2002).unwrap_or(false),
        "cid" => true,
        "iyr" => n.map(|n| n >= 2010 && n <= 2020).unwrap_or(false),
        "eyr" => n.map(|n| n >= 2020 && n <= 2030).unwrap_or(false),
        "hgt" => {
            let n = v
                .chars()
                .take(v.len() - 2)
                .collect::<String>()
                .parse::<i32>();
            if v.ends_with("in") {
                n.map(|n| n >= 59 && n <= 76).unwrap_or(false)
            } else if v.ends_with("cm") {
                n.map(|n| n >= 150 && n <= 193).unwrap_or(false)
            } else {
                false
            }
        }
        "hcl" => {
            lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new("#[0-9a-f]{6}").unwrap();

            }
            RE.is_match(v)
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
        "pid" => {
            v.len() == 9
                && n.map(|n| format!("{:09}", n).parse::<i32>() == Ok(n)) // weak
                    .unwrap_or(false)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"##;

    #[test]
    fn test_something() {
        assert_eq!(2, solve_part1(&input_generator(INPUT)));
    }

    #[test]
    fn test_valid() {
        assert!(validate("byr", "2002")); //valid:
        assert!(!validate("byr", "2003")); //invalid:
        assert!(validate("hgt", "60in")); //valid:
        assert!(validate("hgt", "190cm")); //valid:
        assert!(!validate("hgt", "190in")); //invalid:
        assert!(!validate("hgt", "190")); //invalid:
        assert!(validate("hcl", "#123abc")); //valid:
        assert!(!validate("hcl", "#123abz")); //invalid:
        assert!(!validate("hcl", "123abc")); //invalid:
        assert!(validate("ecl", "brn")); //valid:
        assert!(!validate("ecl", "wat")); //invalid:
        assert!(validate("pid", "000000001")); //valid:
        assert!(!validate("pid", "0123456789")); //invalid:
    }
}
