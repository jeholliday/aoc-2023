fn main() {
    let input: Vec<&str> = include_str!("../../inputs/day06.txt")
        .trim()
        .split("\n")
        .collect();
    println!("{:?}", input);
    let time: u64 = input[0]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let dist: u64 = input[1]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let mut min = time;
    let mut max = 0;

    println!("Trying {} possible speeds", time);
    for speed in 1..time {
        let move_time = time - &speed;
        let move_dist = move_time * speed;
        if move_dist > dist {
            if speed < min {
                min = speed;
            }
            if speed > max {
                max = speed;
            }
        }
    }

    println!("{}", max - min + 1);
}
