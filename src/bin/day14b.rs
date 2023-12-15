use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    RoundRock,
    SquareRock,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Platform {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    cycles: usize,
    history: FxHashMap<Vec<Vec<Tile>>, usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cycle {
    start: usize,
    len: usize,
}

fn get_adjacent_point(platform: &Platform, point: Point, dir: Direction) -> Option<Point> {
    match dir {
        Direction::North => {
            if point.y == 0 {
                None
            } else {
                Some(Point {
                    x: point.x,
                    y: point.y - 1,
                })
            }
        }
        Direction::South => {
            if point.y == platform.height - 1 {
                None
            } else {
                Some(Point {
                    x: point.x,
                    y: point.y + 1,
                })
            }
        }
        Direction::East => {
            if point.x == platform.width - 1 {
                None
            } else {
                Some(Point {
                    x: point.x + 1,
                    y: point.y,
                })
            }
        }
        Direction::West => {
            if point.x == 0 {
                None
            } else {
                Some(Point {
                    x: point.x - 1,
                    y: point.y,
                })
            }
        }
    }
}

fn calculate_load(platform: &Platform) -> u64 {
    let mut load = 0;
    for (y, row) in platform.tiles.iter().enumerate() {
        for tile in row.iter() {
            if *tile == Tile::RoundRock {
                load += platform.height as u64 - y as u64;
            }
        }
    }
    load
}

fn tilt(platform: &mut Platform, dir: Direction) {
    let min_y = match dir {
        Direction::North => 1,
        _ => 0,
    };
    let max_y = match dir {
        Direction::South => platform.height - 1,
        _ => platform.height,
    };
    let min_x = match dir {
        Direction::West => 1,
        _ => 0,
    };
    let max_x = match dir {
        Direction::East => platform.width - 1,
        _ => platform.width,
    };
    let mut moved = 1;
    while moved > 0 {
        moved = 0;
        for y in min_y..max_y {
            for x in min_x..max_x {
                let cur_tile = platform.tiles[y][x];
                if cur_tile != Tile::RoundRock {
                    continue;
                }
                let adjacent_point = match get_adjacent_point(platform, Point { x, y }, dir) {
                    Some(point) => point,
                    None => continue,
                };
                let adjacent_tile = platform.tiles[adjacent_point.y][adjacent_point.x];
                if adjacent_tile == Tile::Empty {
                    platform.tiles[y][x] = Tile::Empty;
                    platform.tiles[adjacent_point.y][adjacent_point.x] = Tile::RoundRock;
                    moved += 1;
                }
            }
        }
    }
}

fn tilt_cycle(platform: &mut Platform) -> Option<Cycle> {
    tilt(platform, Direction::North);
    tilt(platform, Direction::West);
    tilt(platform, Direction::South);
    tilt(platform, Direction::East);

    platform.cycles += 1;
    if let Some(start) = platform.history.get_mut(&platform.tiles) {
        let cycle = Cycle {
            start: *start,
            len: platform.cycles - *start,
        };
        return Some(cycle);
    } else {
        platform
            .history
            .insert(platform.tiles.clone(), platform.cycles);
    }

    None
}

/*fn print_platform(platform: &Platform) {
    for row in &platform.tiles {
        for tile in row {
            match tile {
                Tile::RoundRock => print!("O"),
                Tile::SquareRock => print!("#"),
                Tile::Empty => print!("."),
            }
        }
        println!();
    }
}*/

fn parse_input(input: &str) -> Platform {
    let tiles: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Tile::SquareRock,
                    'O' => Tile::RoundRock,
                    '.' => Tile::Empty,
                    _ => panic!("Invalid tile"),
                })
                .collect()
        })
        .collect();
    let width = tiles[0].len();
    let height = tiles.len();
    let mut history = FxHashMap::default();
    history.insert(tiles.clone(), 0);
    Platform {
        tiles,
        width,
        height,
        cycles: 0,
        history,
    }
}

fn solve(input: &str, cycles: usize) -> u64 {
    let mut platform = parse_input(input);
    //println!("Platform Before:");
    //print_platform(&platform);

    let mut found_cycle: Option<Cycle> = None;
    for cycle in 1..=cycles {
        //println!("\nAfter 1 cycle {}", cycle);
        if let Some(cycle) = tilt_cycle(&mut platform) {
            if found_cycle.is_none() {
                println!("Found cycle starting at {} with length {}", cycle.start, cycle.len);
                found_cycle = Some(cycle);
            }
        }
        if let Some(found_cycle) = found_cycle {
            let cur_cycle_num = found_cycle.start + (cycle - found_cycle.start) % found_cycle.len;
            let wanted_cycle_num =
                found_cycle.start + (cycles - found_cycle.start) % found_cycle.len;
            if cur_cycle_num == wanted_cycle_num {
                break;
            }
        }
        //println!("\nPlatform After:");
        //print_platform(&platform);
    }

    calculate_load(&platform)
}

fn main() {
    let input = include_str!("../../inputs/day14.txt");
    let ans = solve(input, 1000000000);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....";
        let mut platform = parse_input(input);

        let one_cycle = "
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....";
        let one_cycle_platform = parse_input(one_cycle);
        tilt_cycle(&mut platform);
        assert_eq!(platform.tiles, one_cycle_platform.tiles);

        let two_cycles = "
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O";
        let two_cycles_platform = parse_input(two_cycles);
        tilt_cycle(&mut platform);
        assert_eq!(platform.tiles, two_cycles_platform.tiles);

        let three_cycles = "
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O";
        let three_cycles_platform = parse_input(three_cycles);
        tilt_cycle(&mut platform);
        assert_eq!(platform.tiles, three_cycles_platform.tiles);

        assert_eq!(solve(input, 1000000000), 64);
    }
}
