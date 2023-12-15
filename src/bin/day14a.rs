#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    RoundRock,
    SquareRock,
    Empty,
}

fn calculate_load(tiles: &Vec<Vec<Tile>>) -> u64 {
    let mut load = 0;
    for (y, row) in tiles.iter().enumerate() {
        for tile in row.iter() {
            if *tile == Tile::RoundRock {
                load += tiles.len() as u64 - y as u64;
            }
        }
    }
    load
}

fn tilt_north(tiles: &mut Vec<Vec<Tile>>) {
    let mut moved = 1;
    while moved > 0 {
        moved = 0;
        // Skip first row
        for y in 1..tiles.len() {
            for x in 0..tiles[y].len() {
                let cur_tile = tiles[y][x];
                let above_tile = tiles[y - 1][x];
                if cur_tile == Tile::RoundRock && above_tile == Tile::Empty {
                    tiles[y][x] = Tile::Empty;
                    tiles[y - 1][x] = Tile::RoundRock;
                    moved += 1;
                }
            }
        }
    }
}

fn print_tiles(tiles: &Vec<Vec<Tile>>) {
    for row in tiles {
        for tile in row {
            match tile {
                Tile::RoundRock => print!("O"),
                Tile::SquareRock => print!("#"),
                Tile::Empty => print!("."),
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
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
        .collect()
}

fn solve(input: &str) -> u64 {
    let mut tiles = parse_input(input);
    println!("Tiles Before:");
    print_tiles(&tiles);

    tilt_north(&mut tiles);
    println!("\nTiles After:");
    print_tiles(&tiles);

    calculate_load(&tiles)
}

fn main() {
    let input = include_str!("../../inputs/day14.txt");
    let ans = solve(input);
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
        let mut tiles = parse_input(input);
        tilt_north(&mut tiles);

        let tilted_input = "
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....";
        let tilted_tiles = parse_input(tilted_input);
        assert_eq!(tiles, tilted_tiles);

        assert_eq!(calculate_load(&tiles), 136);

        assert_eq!(solve(input), 136);
    }
}
