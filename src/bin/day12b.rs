use rustc_hash::FxHashMap;
use std::fmt::{Debug, Formatter, Result};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct SubProblem {
    springs: Vec<Spring>,
    nums: Vec<u64>,
}

#[derive(PartialEq, Eq, Clone)]
struct Problem {
    items: SubProblem,
    total_working_springs: usize,
    known_working_springs: usize,
    cache: FxHashMap<SubProblem, u64>,
    cache_hits: usize,
    cache_misses: usize,
}

impl ToString for Problem {
    fn to_string(&self) -> String {
        let mut springs: Vec<_> = self
            .items
            .springs
            .iter()
            .map(|s| match s {
                Spring::Operational => "#",
                Spring::Damaged => ".",
                Spring::Unknown => "?",
            })
            .collect();
        springs.reverse();
        let springs: String = springs.join("");
        let mut nums: Vec<_> = self.items.nums.iter().map(|n| n.to_string()).collect();
        nums.reverse();
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
    //let problem_str = problem.to_string();
    if let Some(ans) = problem.cache.get(&problem.items) {
        problem.cache_hits += 1;
        return *ans;
    }
    problem.cache_misses += 1;
    //println!("{:?}", problem);
    if problem.items.springs.len() == 0 {
        if problem.items.nums.len() == 0 {
            //println!("{:?} -> 1", problem);
            problem.cache.insert(problem.items.clone(), 1);
            return 1;
        } else {
            //println!("{:?} -> 0", problem);
            problem.cache.insert(problem.items.clone(), 0);
            return 0;
        }
    }
    let mut ans = 0;
    match problem.items.springs.pop().unwrap() {
        Spring::Operational => {
            // Next num springs must be operational/unknown or we can't continue
            if problem.items.nums.len() > 0 {
                let num = problem.items.nums.pop().unwrap();
                let mut num_operational = 1;
                let mut num_unknown = 0;
                if problem.items.springs.len() >= num as usize - 1 {
                    let removed_springs: Vec<_> = problem
                        .items
                        .springs
                        .drain(problem.items.springs.len() - (num as usize - 1)..)
                        .collect();
                    if removed_springs.len() != num as usize - 1 {
                        panic!("wrong number of springs removed");
                    }
                    for spring in removed_springs.iter() {
                        match spring {
                            Spring::Operational => num_operational += 1,
                            Spring::Unknown => num_unknown += 1,
                            _ => (),
                        }
                    }
                    if num_operational + num_unknown == num {
                        if problem.known_working_springs + num_unknown as usize
                            <= problem.total_working_springs
                        {
                            problem.known_working_springs += num_unknown as usize;
                            match problem.items.springs.last() {
                                Some(Spring::Operational) => {
                                    // Not valid because next is also operational
                                }
                                Some(Spring::Unknown) => {
                                    // Unknown must be non-operational
                                    *problem.items.springs.last_mut().unwrap() = Spring::Damaged;
                                    ans += solve_problem(problem);
                                    *problem.items.springs.last_mut().unwrap() = Spring::Unknown;
                                }
                                _ => {
                                    // Otherwise, non-operational or end of springs
                                    ans += solve_problem(problem);
                                }
                            }
                            problem.known_working_springs -= num_unknown as usize;
                        }
                    }
                    if removed_springs.len() > 0 {
                        problem.items.springs.extend(removed_springs);
                    }
                }
                problem.items.nums.push(num);
            }
            problem.items.springs.push(Spring::Operational);
        }
        Spring::Damaged => {
            // Skip damaged
            let mut num_damaged = 1;
            while problem.items.springs.len() > 0
                && *problem.items.springs.last().unwrap() == Spring::Damaged
            {
                problem.items.springs.pop();
                num_damaged += 1;
            }
            ans += solve_problem(problem);
            for _ in 0..num_damaged {
                problem.items.springs.push(Spring::Damaged);
            }
        }
        Spring::Unknown => {
            // Try both
            if problem.known_working_springs < problem.total_working_springs {
                problem.items.springs.push(Spring::Operational);
                problem.known_working_springs += 1;
                ans += solve_problem(problem);
                problem.known_working_springs -= 1;
                problem.items.springs.pop();
            }
            problem.items.springs.push(Spring::Damaged);
            ans += solve_problem(problem);
            problem.items.springs.pop();
            problem.items.springs.push(Spring::Unknown);
        }
    };

    /*if problem_str != problem.to_string() {
        //println!("{} -> {}", problem_str, problem.to_string());
        panic!("problem changed");
    }*/
    problem.cache.insert(problem.items.clone(), ans);
    return ans;
}

fn solve(input: &str) -> u64 {
    let mut problem = Problem {
        items: SubProblem {
            springs: Vec::new(),
            nums: Vec::new(),
        },
        total_working_springs: 0,
        known_working_springs: 0,
        cache: FxHashMap::default(),
        cache_hits: 0,
        cache_misses: 0,
    };

    let ans = input
        .trim()
        .lines()
        .map(|line| {
            let line: Vec<_> = line.trim().split(" ").collect();
            let orig_springs: Vec<_> = line[0]
                .chars()
                .map(|c| match c {
                    '#' => Spring::Operational,
                    '.' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("invalid spring"),
                })
                .collect();

            let mut springs = orig_springs.clone();
            springs.push(Spring::Unknown);
            springs.extend(orig_springs.iter().cloned());
            springs.push(Spring::Unknown);
            springs.extend(orig_springs.iter().cloned());
            springs.push(Spring::Unknown);
            springs.extend(orig_springs.iter().cloned());
            springs.push(Spring::Unknown);
            springs.extend(orig_springs.iter().cloned());

            springs.reverse();

            let orig_nums: Vec<_> = line[1]
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            let mut nums = orig_nums.clone();
            nums.extend(orig_nums.iter().cloned());
            nums.extend(orig_nums.iter().cloned());
            nums.extend(orig_nums.iter().cloned());
            nums.extend(orig_nums.iter().cloned());

            nums.reverse();

            let total_working_springs: u64 = nums.iter().sum();
            let known_working_springs: u64 = springs
                .iter()
                .filter(|s| **s == Spring::Operational)
                .count() as u64;

            problem.items = SubProblem { springs, nums };
            problem.total_working_springs = total_working_springs as usize;
            problem.known_working_springs = known_working_springs as usize;

            let ans = solve_problem(&mut problem);
            //println!("{:?} -> {}", problem, ans);
            return ans;
        })
        .sum();
    // print hit ratio
    println!(
        "cache hits: {}, cache misses: {}, hit ratio: {}",
        problem.cache_hits,
        problem.cache_misses,
        problem.cache_hits as f64 / (problem.cache_hits + problem.cache_misses) as f64
    );
    return ans;
}

fn main() {
    let input = include_str!("../../inputs/day12.txt");
    let ans = solve(input);
    assert_eq!(ans, 1566786613613);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1";
        assert_eq!(solve(input), 525152);
    }
}
