use rustc_hash::FxHashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterVertical,
    SplitterHorizontal,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::MirrorForward,
            '\\' => Tile::MirrorBackward,
            '|' => Tile::SplitterVertical,
            '-' => Tile::SplitterHorizontal,
            _ => panic!("Invalid tile char: {}", c),
        }
    }
}

fn get_next_directions(cur_dir: &Direction, tile: &Tile) -> Vec<Direction> {
    match tile {
        Tile::Empty => vec![cur_dir.clone()],
        Tile::MirrorForward => match cur_dir {
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
        },
        Tile::MirrorBackward => match cur_dir {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
        },
        Tile::SplitterVertical => match cur_dir {
            Direction::Up => vec![Direction::Up],
            Direction::Down => vec![Direction::Down],
            Direction::Left => vec![Direction::Up, Direction::Down],
            Direction::Right => vec![Direction::Up, Direction::Down],
        },
        Tile::SplitterHorizontal => match cur_dir {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Left],
            Direction::Right => vec![Direction::Right],
        },
    }
}

fn get_energized(grid: &Vec<Vec<Tile>>, initial: (i32, i32, Direction)) -> u64 {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut visited: FxHashSet<(i32, i32, Direction)> = FxHashSet::default();
    let mut to_visit: Vec<(i32, i32, Direction)> = vec![initial];
    while to_visit.len() > 0 {
        let (x, y, dir) = to_visit.pop().unwrap();
        if visited.contains(&(x, y, dir)) {
            continue;
        }
        visited.insert((x, y, dir));
        let tile = grid[y as usize][x as usize];
        let next_dirs = get_next_directions(&dir, &tile);
        for next_dir in next_dirs {
            let (next_x, next_y) = match next_dir {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
            if next_x < 0 || next_y < 0 || next_x >= width || next_y >= height {
                continue;
            }
            to_visit.push((next_x, next_y, next_dir));
        }
    }
    let visited: FxHashSet<(i32, i32)> = visited.iter().map(|(x, y, _)| (*x, *y)).collect();
    return visited.len() as u64;
}

fn solve(input: &str) -> u64 {
    let grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().map(Tile::from_char).collect())
        .collect();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut max = 0;
    for y in 0..height {
        let energized = get_energized(&grid, (0, y, Direction::Right));
        if energized > max {
            max = energized;
        }
        let energized = get_energized(&grid, (width - 1, y, Direction::Left));
        if energized > max {
            max = energized;
        }
    }
    for x in 0..width {
        let energized = get_energized(&grid, (x, 0, Direction::Down));
        if energized > max {
            max = energized;
        }
        let energized = get_energized(&grid, (x, height - 1, Direction::Up));
        if energized > max {
            max = energized;
        }
    }
    return max;
}

fn main() {
    let input = include_str!("../../inputs/day16.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....";
        assert_eq!(solve(input), 51);
    }
}
