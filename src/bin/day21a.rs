use std::str::FromStr;

use rustc_hash::FxHashSet;

#[derive(PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Garden {
    tiles: Vec<Vec<Tile>>,
    start: Point,
    width: usize,
    height: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn get_adjacent(&self, width: usize, height: usize) -> Vec<Point> {
        let mut adjacent = Vec::with_capacity(4);
        if self.x > 0 {
            adjacent.push(Point::new(self.x - 1, self.y));
        }
        if self.x < width - 1 {
            adjacent.push(Point::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            adjacent.push(Point::new(self.x, self.y - 1));
        }
        if self.y < height - 1 {
            adjacent.push(Point::new(self.x, self.y + 1));
        }
        adjacent
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let tiles: Vec<Vec<Tile>> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Plot,
                        '#' => Tile::Rock,
                        'S' => {
                            start = Some(Point::new(x, y));
                            Tile::Plot
                        }
                        _ => panic!("Invalid tile"),
                    })
                    .collect()
            })
            .collect();
        let start = match start {
            Some(start) => start,
            None => panic!("No start found"),
        };
        let width = tiles[0].len();
        let height = tiles.len();
        Ok(Garden {
            tiles,
            start,
            width,
            height,
        })
    }
}

impl ToString for Garden {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x == self.start.x && y == self.start.y {
                    s.push('S');
                } else {
                    match tile {
                        Tile::Plot => s.push('.'),
                        Tile::Rock => s.push('#'),
                    }
                }
            }
            s.push('\n');
        }
        s
    }
}

fn solve(input: &str, steps: usize) -> usize {
    let garden: Garden = input.parse().unwrap();
    println!("{}", garden.to_string());
    let mut visited: FxHashSet<(Point, usize)> = FxHashSet::default();
    let mut queue = vec![(garden.start, 0)];
    while let Some((point, cur_steps)) = queue.pop() {
        if visited.contains(&(point, cur_steps)) {
            continue;
        }
        visited.insert((point, cur_steps));
        if cur_steps == steps {
            continue;
        }
        for adjacent in point.get_adjacent(garden.width, garden.height) {
            if garden.tiles[adjacent.y][adjacent.x] == Tile::Rock {
                continue;
            }
            queue.push((adjacent, cur_steps + 1));
        }
    }
    return visited.iter().filter(|(_, s)| *s == steps).count();
}

fn main() {
    let input = include_str!("../../inputs/day21.txt");
    let ans = solve(input, 64);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........";
        assert_eq!(solve(input, 6), 16);
    }
}
