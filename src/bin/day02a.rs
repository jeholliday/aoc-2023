fn main() {
    let ans = include_str!("../../inputs/day02.txt")
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let line = line.split(":").collect::<Vec<_>>();
            let game = line[0].split(" ").last().unwrap().parse::<u32>().unwrap();
            let hands = line[1].trim().split(";").collect::<Vec<_>>();
            for hand in hands {
                let items = hand.split(",").collect::<Vec<_>>();
                for item in items {
                    let item = item.trim().split(" ").collect::<Vec<_>>();
                    let num = item[0].parse::<u32>().unwrap();
                    let color = item[1];
                    match color {
                        "red" => {
                            if num > 12 {
                                return 0;
                            }
                        }
                        "green" => {
                            if num > 13 {
                                return 0;
                            }
                        }
                        "blue" => {
                            if num > 14 {
                                return 0;
                            }
                        }
                        _ => (),
                    }
                }
            }
            return game;
        })
        .sum::<u32>();
    println!("{}", ans);
}
