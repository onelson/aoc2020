use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Ship {
    facing: Direction,
    location: (i64, i64),
    waypoint: (i64, i64),
}

impl Ship {
    fn new() -> Self {
        Self {
            facing: Direction::East,
            location: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn exec(&mut self, inst: &Instruction) {
        match *inst {
            Instruction::North(v) => {
                self.location.1 += v;
            }
            Instruction::South(v) => {
                self.location.1 -= v;
            }
            Instruction::East(v) => {
                self.location.0 += v;
            }
            Instruction::West(v) => {
                self.location.0 -= v;
            }
            Instruction::Right(v) => {
                self.facing = match self.facing {
                    Direction::North if v == 90 => Direction::East,
                    Direction::North if v == 180 => Direction::South,
                    Direction::North if v == 270 => Direction::West,
                    Direction::South if v == 90 => Direction::West,
                    Direction::South if v == 180 => Direction::North,
                    Direction::South if v == 270 => Direction::East,
                    Direction::East if v == 90 => Direction::South,
                    Direction::East if v == 180 => Direction::West,
                    Direction::East if v == 270 => Direction::North,
                    Direction::West if v == 90 => Direction::North,
                    Direction::West if v == 180 => Direction::East,
                    Direction::West if v == 270 => Direction::South,
                    _ => unreachable!(),
                };
            }
            Instruction::Left(v) => {
                self.facing = match self.facing {
                    Direction::North if v == 90 => Direction::West,
                    Direction::North if v == 180 => Direction::South,
                    Direction::North if v == 270 => Direction::East,
                    Direction::South if v == 90 => Direction::East,
                    Direction::South if v == 180 => Direction::North,
                    Direction::South if v == 270 => Direction::West,
                    Direction::East if v == 90 => Direction::North,
                    Direction::East if v == 180 => Direction::West,
                    Direction::East if v == 270 => Direction::South,
                    Direction::West if v == 90 => Direction::South,
                    Direction::West if v == 180 => Direction::East,
                    Direction::West if v == 270 => Direction::North,
                    _ => unreachable!(),
                };
            }
            Instruction::Forward(v) => match &self.facing {
                Direction::North => {
                    self.location.1 += v;
                }
                Direction::South => {
                    self.location.1 -= v;
                }
                Direction::East => {
                    self.location.0 += v;
                }
                Direction::West => {
                    self.location.0 -= v;
                }
            },
        }
    }

    // Part 2's version with waypoint support.
    fn exec2(&mut self, inst: &Instruction) {
        match *inst {
            Instruction::North(v) => {
                self.waypoint.1 += v;
            }
            Instruction::South(v) => {
                self.waypoint.1 -= v;
            }
            Instruction::East(v) => {
                self.waypoint.0 += v;
            }
            Instruction::West(v) => {
                self.waypoint.0 -= v;
            }
            Instruction::Right(v) => match v {
                90 => {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0);
                }
                180 => {
                    self.waypoint = (-self.waypoint.0, -self.waypoint.1);
                }
                270 => {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0);
                }
                _ => unreachable!(),
            },
            Instruction::Left(v) => match v {
                90 => {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0);
                }
                180 => {
                    self.waypoint = (-self.waypoint.0, -self.waypoint.1);
                }
                270 => {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0);
                }
                _ => unreachable!(),
            },
            Instruction::Forward(v) => {
                let (x1, y1) = self.location;
                let (x2, y2) = self.waypoint;
                self.location = (x1 + (x2 * v), y1 + (y2 * v));
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_at(1);
        let value: i64 = tail.parse().map_err(|_| ())?;
        Ok(match head {
            "N" => Self::North(value),
            "S" => Self::South(value),
            "E" => Self::East(value),
            "W" => Self::West(value),
            "L" => Self::Left(value),
            "R" => Self::Right(value),
            "F" => Self::Forward(value),
            _ => unreachable!(),
        })
    }
}

type Item = Instruction;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Item> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Item]) -> i64 {
    let mut ship = Ship::new();
    // dbg!(&ship);
    for inst in input {
        // dbg!(&inst);
        ship.exec(inst);
        // dbg!(&ship);
    }
    dbg!(ship.location.0.abs() + ship.location.1.abs())
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Item]) -> i64 {
    let mut ship = Ship::new();
    // dbg!(&ship);
    for inst in input {
        // dbg!(&inst);
        ship.exec2(inst);
        // dbg!(&ship);
    }
    dbg!(ship.location.0.abs() + ship.location.1.abs())
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    const INPUT: &str = r##"F10
N3
F7
R90
F11"##;

    #[test]
    fn test_something() {
        assert_eq!(25, solve_part1(&input_generator(INPUT)));
    }

    #[test]
    fn test_something2() {
        assert_eq!(286, solve_part2(&input_generator(INPUT)));
    }
}
