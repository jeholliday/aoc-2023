use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
enum ImagePoint {
    EmptySpace,
    Galaxy,
}

impl ImagePoint {
    fn from_char(c: char) -> ImagePoint {
        match c {
            '.' => ImagePoint::EmptySpace,
            '#' => ImagePoint::Galaxy,
            _ => panic!("Invalid point char: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            ImagePoint::EmptySpace => '.',
            ImagePoint::Galaxy => '#',
        }
    }
}

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

#[derive(PartialEq, Eq, Clone)]
struct Image {
    points: Vec<Vec<ImagePoint>>,
    width: usize,
    height: usize,
}

impl Image {
    fn from_string(s: &str) -> Image {
        let points = s
            .trim()
            .split("\n")
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| ImagePoint::from_char(c))
                    .collect()
            })
            .collect();
        let width = s.trim().split("\n").next().unwrap().len();
        let height = s.trim().split("\n").count();
        Image {
            points,
            width,
            height,
        }
    }

    fn expand(&mut self) {
        let mut cur_row = 0;
        while cur_row < self.height {
            let row_is_all_empty = self.points[cur_row]
                .iter()
                .all(|p| *p == ImagePoint::EmptySpace);
            if row_is_all_empty {
                self.points
                    .insert(cur_row, vec![ImagePoint::EmptySpace; self.width]);
                self.height += 1;
                cur_row += 1;
            }
            cur_row += 1;
        }
        let mut cur_col = 0;
        while cur_col < self.width {
            let col_is_all_empty = self
                .points
                .iter()
                .all(|row| row[cur_col] == ImagePoint::EmptySpace);
            if col_is_all_empty {
                self.points
                    .iter_mut()
                    .for_each(|row| row.insert(cur_col, ImagePoint::EmptySpace));
                self.width += 1;
                cur_col += 1;
            }
            cur_col += 1;
        }
    }

    fn get_galaxies(&self) -> Vec<Point> {
        let mut galaxies = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.points[y][x] == ImagePoint::Galaxy {
                    galaxies.push(Point { x, y });
                }
            }
        }
        galaxies
    }
}

impl ToString for Image {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.points[y][x].to_char());
            }
            s.push('\n');
        }
        s
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

fn solve(input: &str) -> u64 {
    let mut image = Image::from_string(input);
    image.expand();
    println!("{:?}", image);
    let galaxies = image.get_galaxies();
    println!("{:?}", galaxies);
    // for each pair of galaxies, find the manhattan distance between them
    let mut ans = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist = (galaxies[i].x as i32 - galaxies[j].x as i32).abs()
                + (galaxies[i].y as i32 - galaxies[j].y as i32).abs();
            ans += dist as u64;
            println!("{} -> {} = {}", i + 1, j + 1, dist);
        }
    }
    return ans;
}

fn main() {
    let input = include_str!("../../inputs/day11.txt");
    let ans = solve(input);
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
        let input2 = "
            ....#........
            .........#...
            #............
            .............
            .............
            ........#....
            .#...........
            ............#
            .............
            .............
            .........#...
            #....#.......";
        let mut image = Image::from_string(input);
        image.expand();
        let image2 = Image::from_string(input2);
        assert_eq!(image, image2);
        assert_eq!(solve(input), 374);
    }
}
