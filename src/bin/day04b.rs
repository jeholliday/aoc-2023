use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let matching: HashMap<u32, u32> = include_str!("../../inputs/day04.txt")
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return (0, 0);
            }
            let line: Vec<&str> = line.trim().split(" | ").collect();
            let card = line[0]
                .split(": ")
                .nth(0)
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let winning = line[0].split(": ").nth(1).unwrap();
            let winning = winning.split_whitespace().collect::<HashSet<&str>>();
            let mine = line[1].split_whitespace().collect::<HashSet<&str>>();
            let matching = winning.intersection(&mine).collect::<HashSet<&&str>>();
            return (card, matching.len() as u32);
        })
        .collect();
    let mut count: HashMap<u32, u32> = (1..matching.len() as u32).map(|x| (x, 1)).collect();
    for card in 1..matching.len() as u32 {
        let my_matching = matching[&card];
        let my_count = count[&card];
        for i in 1..=my_matching {
            let new_card = card + i;
            let new_count = count[&new_card] + my_count;
            count.insert(new_card, new_count);
        }
        /*println!("Card {}: matching={} count={}", card, my_matching, my_count);
        for card in 1..matching.len() as u32 {
            println!("  Card {}: count={}", card, count[&card]);
        }*/
    }
    let ans: u32 = count.values().sum();
    println!("{}", ans);
}
