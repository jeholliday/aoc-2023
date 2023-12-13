fn main() {
    let ans = include_str!("../../inputs/day02.txt")
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let line = line.split(":").collect::<Vec<_>>();
            let _game = line[0].split(" ").last().unwrap().parse::<u32>().unwrap();
            let hands = line[1].trim().split(";").collect::<Vec<_>>();
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for hand in hands {
                let items = hand.split(",").collect::<Vec<_>>();
                for item in items {
                    let item = item.trim().split(" ").collect::<Vec<_>>();
                    let num = item[0].parse::<u32>().unwrap();
                    let color = item[1];
                    match color {
                        "red" => {
                            if num > min_red {
                                min_red = num;
                            }
                        }
                        "green" => {
                            if num > min_green {
                                min_green = num;
                            }
                        }
                        "blue" => {
                            if num > min_blue {
                                min_blue = num;
                            }
                        }
                        _ => (),
                    }
                }
            }
            return min_red * min_green * min_blue;
        })
        .sum::<u32>();
    println!("{}", ans);
}
