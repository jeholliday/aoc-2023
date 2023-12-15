fn hash(s: &str) -> u64 {
    let mut cur = 0;
    for c in s.chars() {
        cur += c as u64;
        cur *= 17;
        cur = cur % 256;
    }
    cur
}

fn solve(input: &str) -> u64 {
    input.trim().split(',').map(hash).sum()
}

fn main() {
    let input = include_str!("../../inputs/day15.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(hash("HASH".into()), 52);

        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), 1320);
    }
}
