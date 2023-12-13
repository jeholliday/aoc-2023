fn extrapolate(input: &Vec<i64>) -> i64 {
    let is_all_zeros = input.iter().all(|&x| x == 0);
    if is_all_zeros {
        println!("{:?} = 0", input);
        return 0;
    }
    let diffs: Vec<i64> = input.windows(2).map(|x| x[1] - x[0]).collect();
    let first_value = input.first().unwrap();
    let diff = extrapolate(&diffs);
    println!("{:?} = {}", input, first_value - diff);
    return first_value - diff;
}

fn solve(input: &str) -> i64 {
    let input: Vec<&str> = input.trim().split("\n").collect();
    let input: Vec<Vec<i64>> = input
        .iter()
        .map(|x| {
            x.trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let ans: i64 = input.iter().map(extrapolate).sum();
    return ans;
}

fn main() {
    let input = include_str!("../../inputs/day09.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(solve(input), 2);
    }
}
