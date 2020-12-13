use std::fmt;
use std::fmt::Write;

#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Floor,
    Occupied,
}

pub type Item = Tile;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Item>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'L' => Tile::Empty,
                    '.' => Tile::Floor,
                    '#' => Tile::Occupied,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    // search touching tiles
    fn by_adjacency(&self, idx: usize) -> Vec<&Tile> {
        let (x, y) = coords_for_idx(idx, self.width);
        let x = x as isize;
        let y = y as isize;
        [
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x + 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x, y + 1),
            (x, y - 1),
        ]
        .iter()
        .filter(|(x, y)| !is_out_of_bounds(*x, *y, self.width, self.height))
        .map(|&(x, y)| {
            // let (x, y) = wrap_coords(x, y, self.width, self.height);
            let idx = idx_for_coords(x as usize, y as usize, self.width);
            &self.tiles[idx]
        })
        .collect()
    }

    // search each direction for non-empties
    fn by_line_of_sight(&self, idx: usize) -> Vec<&Tile> {
        let (x, y) = coords_for_idx(idx, self.width);
        [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ]
        .iter()
        .filter_map(|&direction| self.trace((x, y), direction))
        .collect()
    }

    fn trace(&self, (x, y): (usize, usize), direction: (isize, isize)) -> Option<&Tile> {
        let mut x = x as isize;
        let mut y = y as isize;

        loop {
            x = x + direction.0;
            y = y + direction.1;
            if is_out_of_bounds(x, y, self.width, self.height) {
                return None;
            }
            let tile = &self.tiles[idx_for_coords(x as usize, y as usize, self.width)];
            match tile {
                Tile::Floor => continue,
                Tile::Empty | Tile::Occupied => return Some(tile),
            }
        }
    }

    fn update<'a, F>(&'a self, max_neighbors: usize, find_neighbors: F) -> Grid
    where
        F: Fn(usize) -> Vec<&'a Tile>,
    {
        // - If a seat is empty (L) and there are no occupied seats adjacent to it, the
        //   seat becomes occupied.
        // - If a seat is occupied (#) and four or more seats adjacent to it are also
        //   occupied, the seat becomes empty.
        // - Otherwise, the seat's state does not change.

        let mut next = self.tiles.clone();

        for idx in 0..self.tiles.len() {
            match &self.tiles[idx] {
                Tile::Empty => {
                    if !find_neighbors(idx)
                        .iter()
                        .any(|&tile| tile == &Tile::Occupied)
                    {
                        next[idx] = Tile::Occupied;
                    }
                }
                Tile::Occupied => {
                    if find_neighbors(idx)
                        .iter()
                        .filter(|&&tile| tile == &Tile::Occupied)
                        .count()
                        >= max_neighbors
                    {
                        next[idx] = Tile::Empty;
                    }
                }
                Tile::Floor => {}
            }
        }

        Grid {
            height: self.height,
            width: self.width,
            tiles: next,
        }
    }

    fn get_count(&self, kind: Tile) -> usize {
        self.tiles.iter().filter(|&t| t == &kind).count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.tiles.chunks(self.width) {
            for tile in chunk {
                match tile {
                    Tile::Empty => {
                        f.write_char('L')?;
                    }
                    Tile::Floor => {
                        f.write_char('.')?;
                    }
                    Tile::Occupied => {
                        f.write_char('#')?;
                    }
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<Item>]) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut prev = Grid {
        tiles: input.to_vec().into_iter().flatten().collect(),
        width,
        height,
    };
    loop {
        println!("{}", &prev);

        let next = prev.update(4, |idx| prev.by_adjacency(idx));
        if next == prev {
            return prev.get_count(Tile::Occupied);
        } else {
            prev = next;
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Vec<Item>]) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut prev = Grid {
        tiles: input.to_vec().into_iter().flatten().collect(),
        width,
        height,
    };

    loop {
        println!("{}", &prev);

        let next = prev.update(5, |idx| prev.by_line_of_sight(idx));
        if next == prev {
            return prev.get_count(Tile::Occupied);
        } else {
            prev = next;
        }
    }
}

fn idx_for_coords(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn coords_for_idx(idx: usize, width: usize) -> (usize, usize) {
    let y = idx / width;
    let x = idx - (y * width);
    (x, y)
}

// False if coords exceed the grid.
fn is_out_of_bounds(x: isize, y: isize, width: usize, height: usize) -> bool {
    x < 0 || y < 0 || x as usize >= width || y as usize >= height
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::*;

    const INPUT: &str = r##"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"##;

    #[test]
    fn test_something() {
        assert_eq!(37, solve_part1(&input_generator(INPUT)));
    }

    #[test]
    fn test_something2() {
        assert_eq!(26, solve_part2(&input_generator(INPUT)));
    }

    #[test]
    fn test_coords_for_idx() {
        assert_eq!((0, 0), coords_for_idx(0, 3));
        assert_eq!((1, 0), coords_for_idx(1, 3));
        assert_eq!((2, 0), coords_for_idx(2, 3));
        assert_eq!((0, 1), coords_for_idx(3, 3));
        assert_eq!((1, 1), coords_for_idx(4, 3));
    }

    #[test]
    fn test_idx_for_coords() {
        assert_eq!(0, idx_for_coords(0, 0, 3));
        assert_eq!(1, idx_for_coords(1, 0, 3));
        assert_eq!(2, idx_for_coords(2, 0, 3));
        assert_eq!(3, idx_for_coords(0, 1, 3));
        assert_eq!(4, idx_for_coords(1, 1, 3));
    }
}
