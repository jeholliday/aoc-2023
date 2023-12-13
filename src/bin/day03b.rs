use std::collections::{HashMap, HashSet};

fn main() {
    let shift: Vec<(i32, i32)> = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ]
    .iter()
    .cloned()
    .collect();

    let input: Vec<Vec<char>> = include_str!("../../inputs/day03.txt")
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let height = input.len() as i32;
    let width = input[0].len() as i32;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        let mut num: Option<String> = None;
        let mut adjacent_gears = HashSet::new();
        for (x, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                for shift in shift.iter() {
                    let (new_x, new_y) = (x as i32 + shift.0, y as i32 + shift.1);
                    if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                        continue;
                    }
                    let k = input[new_y as usize][new_x as usize];
                    if k == '*' {
                        adjacent_gears.insert((new_x as usize, new_y as usize));
                    }
                }
                if num.is_none() {
                    num = Some(c.to_string());
                } else {
                    num = Some(num.unwrap() + &c.to_string());
                }
            } else {
                if let Some(num) = num {
                    for gear in adjacent_gears.iter() {
                        gears
                            .entry(*gear)
                            .or_insert(Vec::new())
                            .push(num.parse::<u32>().unwrap());
                    }
                }
                num = None;
                adjacent_gears = HashSet::new();
            }
        }
        if let Some(num) = num {
            for gear in adjacent_gears.iter() {
                gears
                    .entry(*gear)
                    .or_insert(Vec::new())
                    .push(num.parse::<u32>().unwrap());
            }
        }
    }

    for (k, v) in gears.iter() {
        println!("{:?} {:?}", k, v);
    }
    let ans = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().copied().reduce(|a, b| a * b).unwrap())
        .sum::<u32>();
    println!("{}", ans);
}
