use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
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

    fn from_dirs(dir1: Direction, dir2: Direction) -> Pipe {
        match (dir1, dir2) {
            (Direction::North, Direction::South) => Pipe::NS,
            (Direction::South, Direction::North) => Pipe::NS,
            (Direction::East, Direction::West) => Pipe::EW,
            (Direction::West, Direction::East) => Pipe::EW,
            (Direction::North, Direction::East) => Pipe::NE,
            (Direction::East, Direction::North) => Pipe::NE,
            (Direction::North, Direction::West) => Pipe::NW,
            (Direction::West, Direction::North) => Pipe::NW,
            (Direction::South, Direction::West) => Pipe::SW,
            (Direction::West, Direction::South) => Pipe::SW,
            (Direction::South, Direction::East) => Pipe::SE,
            (Direction::East, Direction::South) => Pipe::SE,
            _ => panic!("Invalid directions"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum GroundType {
    Inside,
    Outside,
    Unknown,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TileItem {
    Ground(GroundType),
    Pipe(Pipe),
    Start,
}

impl TileItem {
    fn from_str(c: char) -> TileItem {
        match c {
            '.' => TileItem::Ground(GroundType::Unknown),
            'O' => TileItem::Ground(GroundType::Outside),
            'I' => TileItem::Ground(GroundType::Inside),
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
            TileItem::Ground(GroundType::Unknown) => ".",
            TileItem::Ground(GroundType::Outside) => "O",
            TileItem::Ground(GroundType::Inside) => "I",
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
}

impl Tile {
    fn get_neighbor(&self, dir: &Direction, map: &Map) -> Option<Tile> {
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
                if self.pos.x == map.width - 1 {
                    return None;
                }
                Position {
                    x: self.pos.x + 1,
                    y: self.pos.y,
                }
            }
            Direction::South => {
                if self.pos.y == map.height - 1 {
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
            map.tiles
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
        println!();
    }
}

fn solve(input: &str) -> u64 {
    let input: Vec<&str> = input.trim().split("\n").collect();
    let mut map = Map {
        tiles: Vec::with_capacity(input.len()),
        width: input[0].len() as u32,
        height: input.len() as u32,
    };
    input.iter().enumerate().for_each(|(y, line)| {
        let mut line_vec = Vec::with_capacity(line.len());
        line.trim().chars().enumerate().for_each(|(x, c)| {
            let pos = Position {
                x: x as u32,
                y: y as u32,
            };
            let item = TileItem::from_str(c);
            let tile = Tile { item, pos: pos };
            line_vec.push(tile);
        });
        map.tiles.push(line_vec);
    });
    let start = map
        .tiles
        .iter()
        .flatten()
        .find(|tile| tile.item == TileItem::Start)
        .unwrap()
        .clone();
    map.draw();

    for try_dir in vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let mut path = Vec::new();
        let mut found_start = false;
        let mut current = match start.get_neighbor(&try_dir, &map) {
            Some(tile) => tile,
            None => continue,
        };
        match current.item {
            TileItem::Pipe(_) => (),
            _ => continue,
        };
        let mut dir = try_dir;
        loop {
            path.push(current.pos.clone());
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
                    break;
                }
            };
            match current.get_neighbor(&dir, &map) {
                Some(next) => current = next,
                None => {
                    break;
                }
            };
        }
        if !found_start {
            continue;
        }

        // Replace start with appropriate pipe and remove dead ends
        let start_dir = try_dir;
        let end_dir = dir;
        let path: HashSet<Position> = path.into_iter().collect();
        for y in 0..map.height {
            for x in 0..map.width {
                let pos = Position { x, y };
                let item = {
                    let tile = map
                        .tiles
                        .get(y as usize)
                        .unwrap()
                        .get(x as usize)
                        .unwrap()
                        .clone();
                    tile.item.clone()
                };
                match item {
                    TileItem::Pipe(_) => {
                        if !path.contains(&pos) {
                            map.tiles
                                .get_mut(y as usize)
                                .unwrap()
                                .get_mut(x as usize)
                                .unwrap()
                                .item = TileItem::Ground(GroundType::Unknown);
                        }
                    }
                    TileItem::Start => {
                        let pipe = Pipe::from_dirs(start_dir, end_dir.get_opposite());
                        map.tiles
                            .get_mut(y as usize)
                            .unwrap()
                            .get_mut(x as usize)
                            .unwrap()
                            .item = TileItem::Pipe(pipe);
                    }
                    _ => (),
                }
            }
        }
        map.draw();

        // Fill in inside/outside
        let mut inside = 0;
        for y in 0..map.height {
            let mut count = 0;
            for x in 0..map.width {
                let item = {
                    let tile = map
                        .tiles
                        .get(y as usize)
                        .unwrap()
                        .get(x as usize)
                        .unwrap()
                        .clone();
                    tile.item.clone()
                };
                match item {
                    TileItem::Pipe(Pipe::NS) => count += 1,
                    TileItem::Pipe(Pipe::NE) => count += 1,
                    TileItem::Pipe(Pipe::NW) => count += 1,
                    TileItem::Pipe(_) => (),
                    TileItem::Ground(GroundType::Unknown) => {
                        if count % 2 == 0 {
                            map.tiles
                                .get_mut(y as usize)
                                .unwrap()
                                .get_mut(x as usize)
                                .unwrap()
                                .item = TileItem::Ground(GroundType::Outside);
                        } else {
                            map.tiles
                                .get_mut(y as usize)
                                .unwrap()
                                .get_mut(x as usize)
                                .unwrap()
                                .item = TileItem::Ground(GroundType::Inside);
                            inside += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
        map.draw();
        return inside;
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
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........";
        assert_eq!(solve(input), 4);
        let input = "
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........";
        assert_eq!(solve(input), 4);
        let input = "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...";
        assert_eq!(solve(input), 8);
        let input = "
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve(input), 10);
    }
}
