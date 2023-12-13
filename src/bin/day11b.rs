use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn solve(input: &str, expansion_ratio: usize) -> u64 {
    let mut galaxies: Vec<Point> = Vec::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                galaxies.push(Point { x, y });
            }
        }
    }
    //println!("{:?}", galaxies);

    // Expand empty rows and columns by the expansion ratio
    let mut max_x = galaxies.iter().map(|p| p.x).max().unwrap();
    let mut max_y = galaxies.iter().map(|p| p.y).max().unwrap();
    let mut cur_row = 0;
    while cur_row <= max_y {
        let row_is_all_empty = galaxies.iter().all(|p| p.y != cur_row);
        if row_is_all_empty {
            galaxies.iter_mut().for_each(|p| {
                if p.y > cur_row {
                    p.y += expansion_ratio as usize - 1;
                }
            });
            max_y += expansion_ratio;
            cur_row += expansion_ratio;
        }
        cur_row += 1;
    }
    let mut cur_col = 0;
    while cur_col <= max_x {
        let col_is_all_empty = galaxies.iter().all(|p| p.x != cur_col);
        if col_is_all_empty {
            galaxies.iter_mut().for_each(|p| {
                if p.x > cur_col {
                    p.x += expansion_ratio as usize - 1;
                }
            });
            max_x += expansion_ratio;
            cur_col += expansion_ratio;
        }
        cur_col += 1;
    }

    // for each pair of galaxies, find the manhattan distance between them
    let mut ans = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist = (galaxies[i].x as i32 - galaxies[j].x as i32).abs()
                + (galaxies[i].y as i32 - galaxies[j].y as i32).abs();
            ans += dist as u64;
            //println!("{} -> {} = {}", i + 1, j + 1, dist);
        }
    }
    return ans;
}

fn main() {
    let input = include_str!("../../inputs/day11.txt");
    let ans = solve(input, 1000000);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....";
        assert_eq!(solve(input, 2), 374);
        assert_eq!(solve(input, 10), 1030);
        assert_eq!(solve(input, 100), 8410);
    }
}
