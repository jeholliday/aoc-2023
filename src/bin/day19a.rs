use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

lazy_static! {
    static ref PART_REGEX: Regex =
        Regex::new(r"^\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)}$").unwrap();
    static ref CONDITIONAL_RULE_REGEX: Regex =
        Regex::new(r"^([xmas])([<>])([0-9]+):([a-zA-Z]+)$").unwrap();
}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum RuleTarget {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct ConditionalRule {
    target: RuleTarget,
    condition: Condition,
    value: usize,
    next: String,
}

#[derive(Debug)]
struct AlwaysRule {
    next: String,
}

#[derive(Debug)]
enum Rule {
    Conditional(ConditionalRule),
    Always(AlwaysRule),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Problem {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = PART_REGEX.captures(s).ok_or("invalid part")?;
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let m = caps.get(2).unwrap().as_str().parse().unwrap();
        let a = caps.get(3).unwrap().as_str().parse().unwrap();
        let s = caps.get(4).unwrap().as_str().parse().unwrap();
        Ok(Part { x, m, a, s })
    }
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Condition::LessThan),
            ">" => Ok(Condition::GreaterThan),
            _ => Err(format!("invalid condition '{}'", s)),
        }
    }
}

impl FromStr for RuleTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(RuleTarget::X),
            "m" => Ok(RuleTarget::M),
            "a" => Ok(RuleTarget::A),
            "s" => Ok(RuleTarget::S),
            _ => Err(format!("invalid rule target '{}'", s)),
        }
    }
}

impl FromStr for ConditionalRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = CONDITIONAL_RULE_REGEX
            .captures(s)
            .ok_or("invalid conditional rule")?;
        let target = RuleTarget::from_str(caps.get(1).unwrap().as_str())?;
        let condition = Condition::from_str(caps.get(2).unwrap().as_str())?;
        let value = caps.get(3).unwrap().as_str().parse().unwrap();
        let next = caps.get(4).unwrap().as_str().to_string();
        Ok(ConditionalRule {
            target,
            condition,
            value,
            next,
        })
    }
}

impl FromStr for AlwaysRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AlwaysRule {
            next: s.to_string(),
        })
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(Rule::Conditional(ConditionalRule::from_str(s)?))
        } else {
            Ok(Rule::Always(AlwaysRule::from_str(s)?))
        }
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("{", " ").replace("}", " ").replace(",", " ");
        let mut parts = s.trim().split_whitespace();
        let name = parts.next().unwrap().to_string();
        let mut rules = Vec::new();
        for part in parts {
            rules.push(Rule::from_str(part)?);
        }
        Ok(Workflow { name, rules })
    }
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split("\n\n");
        let workflows = split
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let workflow: Workflow = line.trim().parse().unwrap();
                (workflow.name.clone(), workflow)
            })
            .collect();
        let parts = split
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| line.trim().parse().unwrap())
            .collect();
        Ok(Problem { parts, workflows })
    }
}

impl Part {
    fn meets_conditional_rule(&self, rule: &ConditionalRule) -> bool {
        let value = match rule.target {
            RuleTarget::X => self.x,
            RuleTarget::M => self.m,
            RuleTarget::A => self.a,
            RuleTarget::S => self.s,
        };
        match rule.condition {
            Condition::LessThan => value < rule.value,
            Condition::GreaterThan => value > rule.value,
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn solve(input: &str) -> usize {
    let problem: Problem = input.parse().unwrap();

    let mut accepted: Vec<Part> = Vec::new();
    let mut rejected: Vec<Part> = Vec::new();
    for part in problem.parts.iter() {
        let mut cur_rule = problem.workflows["in"].rules.iter();
        let mut solved = false;
        loop {
            let mut next = cur_rule.next().unwrap();
            while let Rule::Conditional(rule) = next {
                if part.meets_conditional_rule(rule) {
                    if rule.next == "A" {
                        accepted.push(part.clone());
                        solved = true;
                        break;
                    } else if rule.next == "R" {
                        rejected.push(part.clone());
                        solved = true;
                        break;
                    } else {
                        cur_rule = problem.workflows[&rule.next].rules.iter();
                    }
                }
                next = cur_rule.next().unwrap();
            }
            if solved {
                break;
            }
            if let Rule::Always(rule) = next {
                if rule.next == "A" {
                    accepted.push(part.clone());
                    break;
                } else if rule.next == "R" {
                    rejected.push(part.clone());
                    break;
                } else {
                    cur_rule = problem.workflows[&rule.next].rules.iter();
                }
            }
        }
    }

    accepted.iter().map(|part| part.value()).sum()
}

fn main() {
    let input = include_str!("../../inputs/day19.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(solve(input), 19114);
    }
}
