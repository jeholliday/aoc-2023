use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn get_next_dir(&self, dir: Direction) -> Option<Direction> {
        match self {
            Pipe::NS => match dir {
                Direction::North => Some(Direction::South),
                Direction::South => Some(Direction::North),
                _ => None,
            },
            Pipe::EW => match dir {
                Direction::East => Some(Direction::West),
                Direction::West => Some(Direction::East),
                _ => None,
            },
            Pipe::NE => match dir {
                Direction::North => Some(Direction::East),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Pipe::NW => match dir {
                Direction::North => Some(Direction::West),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Pipe::SW => match dir {
                Direction::South => Some(Direction::West),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Pipe::SE => match dir {
                Direction::South => Some(Direction::East),
                Direction::East => Some(Direction::South),
                _ => None,
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TileItem {
    Ground,
    Pipe(Pipe),
    Start,
}

impl TileItem {
    fn from_str(c: char) -> TileItem {
        match c {
            '.' => TileItem::Ground,
            '|' => TileItem::Pipe(Pipe::NS),
            '-' => TileItem::Pipe(Pipe::EW),
            'L' => TileItem::Pipe(Pipe::NE),
            'J' => TileItem::Pipe(Pipe::NW),
            '7' => TileItem::Pipe(Pipe::SW),
            'F' => TileItem::Pipe(Pipe::SE),
            'S' => TileItem::Start,
            _ => panic!("Invalid tile"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            TileItem::Ground => ".",
            TileItem::Pipe(Pipe::NS) => "|",
            TileItem::Pipe(Pipe::EW) => "-",
            TileItem::Pipe(Pipe::NE) => "L",
            TileItem::Pipe(Pipe::NW) => "J",
            TileItem::Pipe(Pipe::SW) => "7",
            TileItem::Pipe(Pipe::SE) => "F",
            TileItem::Start => "S",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq, Clone)]
struct Tile {
    item: TileItem,
    pos: Position,
    map: Rc<RefCell<Map>>,
}

impl Tile {
    fn get_neighbor(&self, dir: &Direction) -> Option<Tile> {
        let pos = match dir {
            Direction::North => {
                if self.pos.y == 0 {
                    return None;
                }
                Position {
                    x: self.pos.x,
                    y: self.pos.y - 1,
                }
            }
            Direction::East => {
                if self.pos.x == self.map.borrow().width - 1 {
                    return None;
                }
                Position {
                    x: self.pos.x + 1,
                    y: self.pos.y,
                }
            }
            Direction::South => {
                if self.pos.y == self.map.borrow().height - 1 {
                    return None;
                }
                Position {
                    x: self.pos.x,
                    y: self.pos.y + 1,
                }
            }
            Direction::West => {
                if self.pos.x == 0 {
                    return None;
                }
                Position {
                    x: self.pos.x - 1,
                    y: self.pos.y,
                }
            }
        };
        return Some(
            self.map
                .borrow()
                .tiles
                .get(pos.y as usize)
                .unwrap()
                .get(pos.x as usize)
                .unwrap()
                .clone(),
        );
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: u32,
    height: u32,
}

impl Map {
    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.tiles.get(y as usize).unwrap().get(x as usize).unwrap();
                print!("{}", tile.item.to_str());
            }
            println!();
        }
    }
}

fn solve(input: &str) -> u64 {
    let input: Vec<&str> = input.trim().split("\n").collect();
    let map: Rc<RefCell<Map>> = Rc::new(RefCell::new(Map {
        tiles: Vec::with_capacity(input.len()),
        width: input[0].len() as u32,
        height: input.len() as u32,
    }));
    input.iter().enumerate().for_each(|(y, line)| {
        let mut line_vec = Vec::with_capacity(line.len());
        line.trim().chars().enumerate().for_each(|(x, c)| {
            let pos = Position {
                x: x as u32,
                y: y as u32,
            };
            let item = TileItem::from_str(c);
            let tile = Tile {
                item,
                pos: pos,
                map: map.clone(),
            };
            line_vec.push(tile);
        });
        map.borrow_mut().tiles.push(line_vec);
    });
    let start = map
        .borrow()
        .tiles
        .iter()
        .flatten()
        .find(|tile| tile.item == TileItem::Start)
        .unwrap()
        .clone();
    map.borrow().draw();
    println!("Start at x={} y={}", start.pos.x, start.pos.y);
    for try_dir in vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let mut found_start = false;
        let mut steps = 1;
        let mut current = start.get_neighbor(&try_dir).unwrap();
        match current.item {
            TileItem::Pipe(_) => (),
            _ => continue,
        };
        let mut dir = try_dir;
        loop {
            println!(
                "Current at x={} y={} dir={:?}",
                current.pos.x, current.pos.y, dir
            );
            let pipe = match current.item {
                TileItem::Pipe(pipe) => pipe,
                TileItem::Start => {
                    found_start = true;
                    break;
                }
                _ => panic!("Invalid tile"),
            };
            dir = dir.get_opposite();
            dir = match pipe.get_next_dir(dir) {
                Some(next_dir) => next_dir,
                None => {
                    println!("No next dir");
                    break;
                }
            };
            match current.get_neighbor(&dir) {
                Some(next) => current = next,
                None => {
                    println!("No next tile");
                    break;
                }
            };
            steps += 1;
        }
        if found_start {
            return steps / 2;
        }
    }
    panic!("No solution found");
}

fn main() {
    let input = include_str!("../../inputs/day10.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            .....
            .S-7.
            .|.|.
            .L-J.
            .....";
        assert_eq!(solve(input), 4);
        let input = "
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF";
        assert_eq!(solve(input), 4);
        let input = "
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...";
        assert_eq!(solve(input), 8);
        let input = "
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ";
        assert_eq!(solve(input), 8);
    }
}
