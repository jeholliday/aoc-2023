use std::collections::HashMap;

fn main() {
    let digits: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let ans = include_str!("../../inputs/day01.txt")
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let mut first = None;
            let mut first_index = None;
            let mut last = None;
            let mut last_index = None;

            for (k, v) in digits.iter() {
                let indices = line.match_indices(k);
                for (i, _) in indices {
                    if first_index.is_none() || i < first_index.unwrap() {
                        first_index = Some(i);
                        first = Some(v.to_string());
                    }
                    if last_index.is_none() || i > last_index.unwrap() {
                        last_index = Some(i);
                        last = Some(v.to_string());
                    }
                }
            }

            (first.unwrap() + &last.unwrap()).parse::<u32>().unwrap()
        })
        .sum::<u32>();
    println!("{}", ans);
}
