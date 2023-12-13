use std::collections::HashSet;

fn main() {
    let ans = include_str!("../../inputs/day04.txt")
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let line: Vec<&str> = line.trim().split(" | ").collect();
            let card = line[0].split(": ").nth(0).unwrap();
            let winning = line[0].split(": ").nth(1).unwrap();
            let winning = winning.split_whitespace().collect::<HashSet<&str>>();
            let mine = line[1].split_whitespace().collect::<HashSet<&str>>();
            let matching = winning.intersection(&mine).collect::<HashSet<&&str>>();
            let points = {
                if matching.len() == 0 {
                    0
                } else {
                    2u32.pow((matching.len() - 1) as u32)
                }
            };
            println!(
                "{}: {} matching for {} points",
                card,
                matching.len(),
                points
            );
            points
        })
        .sum::<u32>();
    println!("{}", ans);
}
