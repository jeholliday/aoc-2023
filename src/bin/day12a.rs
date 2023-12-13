use std::fmt::{Debug, Formatter, Result};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(PartialEq, Eq, Clone)]
struct Problem {
    springs: Vec<Spring>,
    nums: Vec<u64>,
    //total_working_springs: usize,
    //known_working_springs: usize,
}

impl ToString for Problem {
    fn to_string(&self) -> String {
        let springs: String = self
            .springs
            .iter()
            .map(|s| match s {
                Spring::Operational => "#",
                Spring::Damaged => ".",
                Spring::Unknown => "?",
            })
            .collect();
        let nums: Vec<_> = self.nums.iter().map(|n| n.to_string()).collect();
        let nums: String = nums.join(",");
        format!("[{} {}]", springs, nums)
    }
}

impl Debug for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

fn solve_problem(problem: &mut Problem) -> u64 {
    //println!("{:?}", problem);
    if problem.springs.len() == 0 {
        if problem.nums.len() == 0 {
            //println!("{:?} -> 1", problem);
            return 1;
        } else {
            //println!("{:?} -> 0", problem);
            return 0;
        }
    }
    let mut ans = 0;
    match problem.springs.remove(0) {
        Spring::Operational => {
            // Next num springs must be operational/unknown or we can't continue
            if problem.nums.len() > 0 {
                let num = problem.nums.remove(0);
                let mut num_operational = 1;
                let mut num_unknown = 0;
                if problem.springs.len() >= num as usize - 1 {
                    let removed_springs: Vec<_> =
                        problem.springs.drain(..num as usize - 1).collect();
                    for spring in removed_springs.iter() {
                        match spring {
                            Spring::Operational => num_operational += 1,
                            Spring::Unknown => num_unknown += 1,
                            _ => (),
                        }
                    }
                    if num_operational + num_unknown == num {
                        match problem.springs.get(0) {
                            Some(Spring::Operational) => {
                                // Not valid because next is also operational
                            }
                            Some(Spring::Unknown) => {
                                // Unknown must be non-operational
                                problem.springs[0] = Spring::Damaged;
                                ans += solve_problem(problem);
                                problem.springs[0] = Spring::Unknown;
                            }
                            _ => {
                                // Otherwise, non-operational or end of springs
                                ans += solve_problem(problem);
                            }
                        }
                    }
                    problem.springs.splice(0..0, removed_springs);
                }
                problem.nums.insert(0, num);
            }
            problem.springs.insert(0, Spring::Operational);
        }
        Spring::Damaged => {
            // Skip
            ans += solve_problem(problem);
            problem.springs.insert(0, Spring::Damaged);
        }
        Spring::Unknown => {
            // Try both
            problem.springs.insert(0, Spring::Operational);
            ans += solve_problem(problem);
            problem.springs.remove(0);
            problem.springs.insert(0, Spring::Damaged);
            ans += solve_problem(problem);
            problem.springs.remove(0);
            problem.springs.insert(0, Spring::Unknown);
        }
    };

    //println!("{:?} -> {}", problem, ans);
    return ans;
}

fn solve_line(line: &str) -> u64 {
    let line: Vec<_> = line.split(" ").collect();
    let springs: Vec<_> = line[0]
        .chars()
        .map(|c| match c {
            '#' => Spring::Operational,
            '.' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("invalid spring"),
        })
        .collect();
    let nums: Vec<_> = line[1]
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    //let total_working_springs: u64 = nums.iter().sum();
    //let known_working_springs: u64 = springs.iter().filter(|s| **s == Spring::Operational).count() as u64;
    let mut problem = Problem {
        springs,
        nums,
        //total_working_springs: total_working_springs as usize,
        //known_working_springs: known_working_springs as usize,
    };
    solve_problem(&mut problem)
}

fn solve(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| solve_line(line.trim()))
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day12.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve_line("???.### 1,1,3"), 1);
        assert_eq!(solve_line(".??..??...?##. 1,1,3"), 4);
        assert_eq!(solve_line("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(solve_line("????.#...#... 4,1,1"), 1);
        assert_eq!(solve_line("????.######..#####. 1,6,5"), 4);
        assert_eq!(solve_line("?###???????? 3,2,1"), 10);
        let input = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1";
        assert_eq!(solve(input), 21);
    }
}
