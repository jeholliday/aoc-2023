use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BrickType {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
    start: Point,
    len: usize,
    brick_type: BrickType,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',');
        let x = coords.next().ok_or("no x").unwrap().parse()?;
        let y = coords.next().ok_or("no y").unwrap().parse()?;
        let z = coords.next().ok_or("no z").unwrap().parse()?;
        Ok(Point { x, y, z })
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split('~');
        let a: Point = points.next().unwrap().parse().unwrap();
        let b: Point = points.next().unwrap().parse().unwrap();
        assert!(b >= a);
        if a.x == b.x {
            if a.y == b.y {
                let len = b.z - a.z + 1;
                Ok(Brick {
                    start: a,
                    len,
                    brick_type: BrickType::Z,
                })
            } else if a.z == b.z {
                let len = b.y - a.y + 1;
                Ok(Brick {
                    start: a,
                    len,
                    brick_type: BrickType::Y,
                })
            } else {
                Err(())
            }
        } else if a.y == b.y {
            if a.z == b.z {
                let len = b.x - a.x + 1;
                Ok(Brick {
                    start: a,
                    len,
                    brick_type: BrickType::X,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl Brick {
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::with_capacity(self.len);
        match self.brick_type {
            BrickType::X => {
                for x in self.start.x..self.start.x + self.len {
                    points.push(Point {
                        x,
                        y: self.start.y,
                        z: self.start.z,
                    });
                }
            }
            BrickType::Y => {
                for y in self.start.y..self.start.y + self.len {
                    points.push(Point {
                        x: self.start.x,
                        y,
                        z: self.start.z,
                    });
                }
            }
            BrickType::Z => {
                for z in self.start.z..self.start.z + self.len {
                    points.push(Point {
                        x: self.start.x,
                        y: self.start.y,
                        z,
                    });
                }
            }
        }
        points
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BrickInfo {
    id: usize,
    brick: Brick,
    supports: Vec<BrickRef>,
    supported_by: Vec<BrickRef>,
}

type BrickRef = Rc<RefCell<BrickInfo>>;

fn solve(input: &str) -> usize {
    let bricks: Vec<Brick> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let max_x = bricks
        .iter()
        .map(|b| b.points().into_iter().map(|p| p.x).max().unwrap())
        .max()
        .unwrap();
    let max_y = bricks
        .iter()
        .map(|b| b.points().into_iter().map(|p| p.y).max().unwrap())
        .max()
        .unwrap();
    let max_z = bricks
        .iter()
        .map(|b| b.points().into_iter().map(|p| p.z).max().unwrap())
        .max()
        .unwrap();

    let mut grid: Vec<Vec<Vec<Option<BrickRef>>>> =
        vec![vec![vec![None; max_z + 1]; max_y + 1]; max_x + 1];
    let bricks: Vec<BrickRef> = bricks
        .into_iter()
        .enumerate()
        .map(|(id, brick)| {
            let brick = BrickInfo {
                id,
                brick,
                supports: Vec::new(),
                supported_by: Vec::new(),
            };
            Rc::new(RefCell::new(brick))
        })
        .collect();
    let mut to_visit = bricks.clone();
    while to_visit.len() > 0 {
        let lowest = to_visit
            .iter()
            .min_by_key(|b| b.borrow().brick.start.z)
            .unwrap()
            .clone();
        to_visit.retain(|b| !Rc::ptr_eq(b, &lowest));
        let cur = lowest;
        loop {
            let points = {
                let cur = cur.borrow();
                match cur.brick.brick_type {
                    BrickType::X => cur.brick.points(),
                    BrickType::Y => cur.brick.points(),
                    BrickType::Z => vec![cur.brick.start.clone()],
                }
            };
            // Check if each point can move down by one
            let mut can_move_down = true;
            for point in &points {
                if point.z == 1 {
                    can_move_down = false;
                } else if let Some(below) = grid[point.x][point.y][point.z - 1].as_ref() {
                    can_move_down = false;
                    {
                        let mut cur = cur.borrow_mut();
                        if !cur.supported_by.contains(&below) {
                            cur.supported_by.push(below.clone());
                        }
                    }
                    let mut below = below.borrow_mut();
                    if !below.supports.contains(&cur) {
                        below.supports.push(cur.clone());
                    }
                }
            }
            if can_move_down {
                cur.borrow_mut().brick.start.z -= 1;
            } else {
                break;
            }
        }
        assert!(cur.borrow().brick.start.z == 1 || cur.borrow().supported_by.len() > 0);
        for point in cur.borrow().brick.points() {
            grid[point.x][point.y][point.z] = Some(cur.clone());
        }
    }

    bricks
        .iter()
        .filter(|b| {
            let b = b.borrow();
            if b.supports.len() == 0 {
                return true;
            }
            for brick in b.supports.iter() {
                let brick = brick.borrow();
                if brick.supported_by.len() <= 1 {
                    return false;
                }
            }
            return true;
        })
        .count()
}

fn main() {
    let input = include_str!("../../inputs/day22.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9";
        assert_eq!(solve(input), 5);
    }
}
