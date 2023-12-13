use std::collections::HashMap;

fn solve(input: &str) -> u32 {
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut input: Vec<&str> = input.trim().split("\n").collect();
    let dirs: Vec<char> = input.remove(0).trim().chars().collect();
    input.remove(0); // blank line
    for line in input.iter() {
        let line = line.trim().split_whitespace().collect::<Vec<&str>>();
        let node = line[0];
        let left = &line[2][1..4];
        let right = &line[3][0..3];
        graph.insert(node, (left, right));
    }
    let mut node = "AAA";
    let mut steps: u32 = 0;
    while node != "ZZZ" {
        let dir_index = steps as usize % dirs.len();
        let dir = dirs[dir_index];
        let (left, right) = graph.get(node).unwrap();
        match dir {
            'L' => node = left,
            'R' => node = right,
            _ => panic!("Invalid direction"),
        }
        steps += 1;
    }
    return steps;
}

fn main() {
    let input = include_str!("../../inputs/day08.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 2);
        let input = "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 6);
    }
}
