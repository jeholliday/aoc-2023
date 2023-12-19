use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_string(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction '{}'", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    distance: u64,
    color: String,
}

impl Instruction {
    fn from_string(s: &str) -> Instruction {
        let mut parts = s.trim().split_whitespace();
        let direction = Direction::from_string(parts.next().unwrap());
        let distance = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap().to_string();
        let color = color[2..color.len() - 1].to_string();
        Instruction {
            direction,
            distance,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TrenchType {
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    VerticalEdge,
    HorizontalEdge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ClassifiedPoint {
    Trench(TrenchType),
    Inside,
    Outside,
    Unclassified,
}

fn classify_trench(trench: HashSet<Point>, p: Point) -> TrenchType {
    let above = Point { x: p.x, y: p.y + 1 };
    let below = Point { x: p.x, y: p.y - 1 };
    let left = Point { x: p.x - 1, y: p.y };
    let right = Point { x: p.x + 1, y: p.y };
    if trench.contains(&below) && trench.contains(&right) {
        TrenchType::TopLeftCorner
    } else if trench.contains(&below) && trench.contains(&left) {
        TrenchType::TopRightCorner
    } else if trench.contains(&above) && trench.contains(&right) {
        TrenchType::BottomLeftCorner
    } else if trench.contains(&above) && trench.contains(&left) {
        TrenchType::BottomRightCorner
    } else if trench.contains(&above) && trench.contains(&below) {
        TrenchType::VerticalEdge
    } else if trench.contains(&left) && trench.contains(&right) {
        TrenchType::HorizontalEdge
    } else {
        panic!("unclassified trench point {:?}", p);
    }
}

fn solve(input: &str) -> u64 {
    let instrs: Vec<_> = input.trim().lines().map(Instruction::from_string).collect();
    let mut trench: HashSet<Point> = HashSet::new();
    let mut cur = Point { x: 0, y: 0 };
    trench.insert(cur.clone());
    for instr in instrs {
        for _ in 0..instr.distance {
            match instr.direction {
                Direction::Up => cur.y += 1,
                Direction::Down => cur.y -= 1,
                Direction::Left => cur.x -= 1,
                Direction::Right => cur.x += 1,
            }
            trench.insert(cur.clone());
        }
    }
    println!("Trench length = {}", trench.len());
    let min_x = trench.iter().map(|p| p.x).min().unwrap();
    let max_x = trench.iter().map(|p| p.x).max().unwrap();
    let min_y = trench.iter().map(|p| p.y).min().unwrap();
    let max_y = trench.iter().map(|p| p.y).max().unwrap();
    println!(
        "min_x = {}, max_x = {}, min_y = {}, max_y = {}",
        min_x, max_x, min_y, max_y
    );
    // Draw trench
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let p = Point { x, y };
            if trench.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut lagoon: Vec<Vec<ClassifiedPoint>> = (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| {
                    let p = Point { x: x, y: y };
                    if trench.contains(&p) {
                        ClassifiedPoint::Trench(classify_trench(trench.clone(), p))
                    } else {
                        ClassifiedPoint::Unclassified
                    }
                })
                .collect()
        })
        .collect();
    for y in (min_y..=max_y).rev() {
        let mut count = 0;
        for x in min_x..=max_x {
            let p = Point {
                x: x - min_x,
                y: y - min_y,
            };
            match lagoon[p.y as usize][p.x as usize] {
                ClassifiedPoint::Trench(TrenchType::TopLeftCorner) => (),
                ClassifiedPoint::Trench(TrenchType::TopRightCorner) => (),
                ClassifiedPoint::Trench(TrenchType::BottomLeftCorner) => count += 1,
                ClassifiedPoint::Trench(TrenchType::BottomRightCorner) => count += 1,
                ClassifiedPoint::Trench(TrenchType::VerticalEdge) => count += 1,
                ClassifiedPoint::Trench(TrenchType::HorizontalEdge) => (),
                ClassifiedPoint::Unclassified => {
                    if count % 2 == 1 {
                        lagoon[p.y as usize][p.x as usize] = ClassifiedPoint::Inside;
                    } else {
                        lagoon[p.y as usize][p.x as usize] = ClassifiedPoint::Outside;
                    }
                }
                _ => (),
            }
        }
    }

    // draw lagoon
    let mut size = trench.len();
    println!();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let p = Point {
                x: x - min_x,
                y: y - min_y,
            };
            match lagoon[p.y as usize][p.x as usize] {
                ClassifiedPoint::Trench(_) => print!("#"),
                ClassifiedPoint::Inside => {
                    size += 1;
                    print!("#")
                }
                ClassifiedPoint::Outside => print!("."),
                ClassifiedPoint::Unclassified => print!("?"),
            }
        }
        println!();
    }

    return size as u64;
}

fn main() {
    let input = include_str!("../../inputs/day18.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)";
        assert_eq!(solve(input), 62);
    }
}
