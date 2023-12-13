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
    let mut ans = 0;
    for (y, line) in input.iter().enumerate() {
        let mut num: Option<String> = None;
        let mut is_part = false;
        for (x, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                if !is_part {
                    for shift in shift.iter() {
                        let (new_x, new_y) = (x as i32 + shift.0, y as i32 + shift.1);
                        if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                            continue;
                        }
                        let k = input[new_y as usize][new_x as usize];
                        if !k.is_digit(10) && k != '.' {
                            is_part = true;
                            break;
                        }
                    }
                }
                if num.is_none() {
                    num = Some(c.to_string());
                } else {
                    num = Some(num.unwrap() + &c.to_string());
                }
            } else {
                if let Some(num) = num {
                    println!("{} {}", num, is_part);
                    if is_part {
                        ans += num.parse::<i32>().unwrap();
                    }
                }
                num = None;
                is_part = false;
            }
        }
        if let Some(num) = num {
            println!("{} {}", num, is_part);
            if is_part {
                ans += num.parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", ans);
}
