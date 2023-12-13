fn main() {
    let ans = include_str!("../../inputs/day01.txt")
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let chars = line.chars().filter(|c| c.is_digit(10));
            let first = chars.clone().next().unwrap().to_string();
            let last = chars.last().unwrap().to_string();
            (first + &last).parse::<u32>().unwrap()
        })
        .sum::<u32>();
    println!("{}", ans);
}
