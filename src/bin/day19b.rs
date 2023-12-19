use core::panic;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const RATING_MIN_VALUE: usize = 1;
const RATING_MAX_VALUE: usize = 4000;

lazy_static! {
    static ref CONDITIONAL_RULE_REGEX: Regex =
        Regex::new(r"^([xmas])([<>])([0-9]+):([a-zA-Z]+)$").unwrap();
}

#[derive(Debug, Clone)]
struct PartRange {
    min_x: usize,
    max_x: usize,
    min_m: usize,
    max_m: usize,
    min_a: usize,
    max_a: usize,
    min_s: usize,
    max_s: usize,
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
    workflows: HashMap<String, Workflow>,
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
        Ok(Problem { workflows })
    }
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            min_x: RATING_MIN_VALUE,
            max_x: RATING_MAX_VALUE,
            min_m: RATING_MIN_VALUE,
            max_m: RATING_MAX_VALUE,
            min_a: RATING_MIN_VALUE,
            max_a: RATING_MAX_VALUE,
            min_s: RATING_MIN_VALUE,
            max_s: RATING_MAX_VALUE,
        }
    }

    fn apply_conditional_rule(&self, rule: &ConditionalRule) -> Option<PartRange> {
        let mut range = self.clone();
        match rule.target {
            RuleTarget::X => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_x {
                        range.max_x = rule.value - 1;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_x {
                        range.min_x = rule.value + 1;
                    } else {
                        return None;
                    }
                }
            },
            RuleTarget::M => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_m {
                        range.max_m = rule.value - 1;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_m {
                        range.min_m = rule.value + 1;
                    } else {
                        return None;
                    }
                }
            },
            RuleTarget::A => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_a {
                        range.max_a = rule.value - 1;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_a {
                        range.min_a = rule.value + 1;
                    } else {
                        return None;
                    }
                }
            },
            RuleTarget::S => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_s {
                        range.max_s = rule.value - 1;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_s {
                        range.min_s = rule.value + 1;
                    } else {
                        return None;
                    }
                }
            },
        }
        Some(range)
    }

    fn apply_conditional_rule_rev(&self, rule: &ConditionalRule) -> Option<PartRange> {
        let mut range = self.clone();
        match rule.target {
            RuleTarget::X => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_x {
                        range.min_x = rule.value;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_x {
                        range.max_x = rule.value;
                    } else {
                        return None;
                    }
                }
            },
            RuleTarget::M => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_m {
                        range.min_m = rule.value;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_m {
                        range.max_m = rule.value;
                    } else {
                        return None;
                    }
                }
            },
            RuleTarget::A => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_a {
                        range.min_a = rule.value;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_a {
                        range.max_a = rule.value;
                    } else {
                        return None;
                    }
                }
            },
            RuleTarget::S => match rule.condition {
                Condition::LessThan => {
                    if rule.value >= self.min_s {
                        range.min_s = rule.value;
                    } else {
                        return None;
                    }
                }
                Condition::GreaterThan => {
                    if rule.value <= self.max_s {
                        range.max_s = rule.value;
                    } else {
                        return None;
                    }
                }
            },
        }
        Some(range)
    }

    fn intersection(&self, other: &PartRange) -> PartRange {
        PartRange {
            min_x: self.min_x.max(other.min_x),
            max_x: self.max_x.min(other.max_x),
            min_m: self.min_m.max(other.min_m),
            max_m: self.max_m.min(other.max_m),
            min_a: self.min_a.max(other.min_a),
            max_a: self.max_a.min(other.max_a),
            min_s: self.min_s.max(other.min_s),
            max_s: self.max_s.min(other.max_s),
        }
    }

    fn combinations(&self) -> usize {
        (self.max_x - self.min_x + 1)
            * (self.max_m - self.min_m + 1)
            * (self.max_a - self.min_a + 1)
            * (self.max_s - self.min_s + 1)
    }
}

fn solve(input: &str) -> usize {
    let problem: Problem = input.parse().unwrap();
    let mut solved: HashMap<String, Vec<PartRange>> = HashMap::new();
    solved.insert("A".to_string(), vec![PartRange::new()]);
    let mut unsolved: HashSet<String> = problem.workflows.keys().cloned().collect();

    while !solved.contains_key("in") {
        let mut solved_workflow: Option<(String, Vec<PartRange>)> = None;
        for workflow in unsolved.iter() {
            let workflow = problem.workflows.get(workflow).unwrap();
            let mut range = PartRange::new();
            let mut valid = true;
            let mut valid_ranges = Vec::new();
            for rule in workflow.rules.iter() {
                match rule {
                    Rule::Conditional(rule) => {
                        if let Some(solved_ranges) = solved.get(&rule.next) {
                            let conditional_range = range.apply_conditional_rule(rule);
                            for solved_range in solved_ranges.iter() {
                                if let Some(conditional_range) = &conditional_range {
                                    // Range which meets the conditional rule and is solved by the target workflow
                                    let valid_range = conditional_range.intersection(solved_range);
                                    valid_ranges.push(valid_range);
                                }
                            }
                            if let Some(next_range) = range.apply_conditional_rule_rev(rule) {
                                // Range which bypasses conditional rule
                                range = next_range;
                            } else {
                                // No range which bypasses conditional rule, so no more valid ranges
                                break;
                            }
                        } else if rule.next == "R" {
                            // Remove this range and continue
                            if let Some(next_range) = range.apply_conditional_rule_rev(rule) {
                                range = next_range;
                            } else {
                                // No range which bypasses conditional rule, so no more valid ranges
                                break;
                            }
                        } else {
                            valid = false;
                            break;
                        }
                    }
                    Rule::Always(rule) => {
                        if let Some(solved_ranges) = solved.get(&rule.next) {
                            for solved_range in solved_ranges.iter() {
                                let valid_range = range.intersection(solved_range);
                                valid_ranges.push(valid_range);
                            }
                            // Range which is solved by the target workflow
                            break;
                        } else if rule.next == "R" {
                            // No more valid range, but there might have been previous valid ranges
                            break;
                        } else {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            if valid {
                solved_workflow = Some((workflow.name.clone(), valid_ranges));
                break;
            }
        }
        if let Some((workflow, valid_ranges)) = solved_workflow {
            println!("solved {}", workflow);
            unsolved.remove(&workflow);
            solved.insert(workflow, valid_ranges);
        } else {
            panic!("no solution found");
        }
    }

    let valid_ranges = solved.get("in").unwrap();
    valid_ranges.iter().map(|range| range.combinations()).sum()
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
        assert_eq!(solve(input), 167409079868000);
    }
}
