fn main() {
    let input: Vec<&str> = include_str!("../../inputs/day06.txt")
        .trim()
        .split("\n")
        .collect();
    println!("{:?}", input);
    let time = input[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let dist = input[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let ans = time
        .iter()
        .zip(dist.iter())
        .map(|(t, d)| {
            let mut min = *t;
            let mut max = 0;

            for speed in 1..*t {
                let move_time = t - &speed;
                let move_dist = move_time * speed;
                if move_dist > *d {
                    if speed < min {
                        min = speed;
                    }
                    if speed > max {
                        max = speed;
                    }
                }
            }
            println!("{} {}", min, max);

            return max - min + 1;
        })
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{}", ans);
}
