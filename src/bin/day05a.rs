fn main() {
    let mut rules: Vec<&str> = include_str!("../../inputs/day05.txt")
        .trim()
        .split("\n\n")
        .collect();
    let seeds: Vec<u64> = rules
        .remove(0)
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut items = seeds.clone();
    for rule in rules {
        let mut new_items = Vec::new();
        let mut sub_rules: Vec<&str> = rule.split("\n").collect();
        sub_rules.remove(0);
        for item in items {
            let mut found = false;
            for sub_rule in &sub_rules {
                let nums: Vec<u64> = sub_rule
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                let dst = nums[0];
                let src = nums[1];
                let len = nums[2];
                if item >= src && item < src + len {
                    let mapped = (item - src) + dst;
                    new_items.push(mapped);
                    found = true;
                    break;
                }
            }
            if !found {
                new_items.push(item);
            }
        }
        items = new_items;
    }
    let ans = items.iter().min().unwrap();
    println!("{}", ans);
}
