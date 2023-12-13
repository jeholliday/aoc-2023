use std::cmp::{max, min};

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    start: u64,
    len: u64,
}

type Ranges = Vec<Range>;

fn merge_ranges(ranges: &Ranges) -> Ranges {
    let mut ranges = ranges.clone();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged_ranges = Vec::new();
    let mut current_range = ranges[0].clone();
    for range in ranges {
        if range.start <= current_range.start + current_range.len {
            current_range.len = max(
                current_range.start + current_range.len,
                range.start + range.len,
            ) - current_range.start;
        } else {
            merged_ranges.push(current_range);
            current_range = range;
        }
    }
    merged_ranges.push(current_range);
    merged_ranges
}

#[derive(Clone, Debug, PartialEq)]
struct RuleRange {
    src: Range,
    dst: Range,
}

type Rule = Vec<RuleRange>;

fn main() {
    let mut rules: Vec<&str> = include_str!("../../inputs/day05.txt")
        .trim()
        .split("\n\n")
        .collect();
    let seeds: Vec<u64> = rules
        .remove(0)
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let seeds: Vec<Range> = seeds
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            len: chunk[1],
        })
        .collect();
    let rules: Vec<Rule> = rules
        .iter()
        .map(|rule| {
            let mut sub_rules: Vec<&str> = rule.split("\n").collect();
            sub_rules.remove(0);
            let sub_rules: Vec<RuleRange> = sub_rules
                .iter()
                .map(|sub_rule| {
                    let nums: Vec<u64> = sub_rule
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
                    let src = Range {
                        start: nums[1],
                        len: nums[2],
                    };
                    let dst = Range {
                        start: nums[0],
                        len: nums[2],
                    };
                    RuleRange { src: src, dst: dst }
                })
                .collect();
            sub_rules
        })
        .collect();
    let mut ranges: Vec<Range> = seeds.clone();
    for rule in rules {
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        let mut new_ranges: Vec<Range> = Vec::new();
        let mut rules_ranges = rule.clone();
        rules_ranges.sort_by(|a, b| a.src.start.cmp(&b.src.start));
        let mut rule_range: usize = 0;
        for range in ranges {
            let mut range = range.clone();
            let mut done = false;
            while rule_range < rules_ranges.len() {
                let rule_start = rules_ranges[rule_range].src.start;
                let rule_end =
                    rules_ranges[rule_range].src.start + rules_ranges[rule_range].src.len;
                let range_start = range.start;
                let range_end = range.start + range.len;
                let start = max(rule_start, range_start);
                let end = min(rule_end, range_end);
                if rule_start > range_end {
                    new_ranges.push(range.clone());
                    done = true;
                    break;
                }
                if rule_end <= range_start {
                    rule_range += 1;
                    continue;
                }
                if range_start < rule_start {
                    /* Part of start of range is before rule */
                    let before = Range {
                        start: range_start,
                        len: rule_start - range_start,
                    };
                    new_ranges.push(before);
                }
                let mapped = Range {
                    start: rules_ranges[rule_range].dst.start
                        + (start - rules_ranges[rule_range].src.start),
                    len: end - start,
                };
                new_ranges.push(mapped);
                if rule_end < range_end {
                    /* Part of end of range is after rule */
                    range = Range {
                        start: rule_end,
                        len: range_end - rule_end,
                    };
                    rule_range += 1;
                } else {
                    /* Check next range with same rule */
                    done = true;
                    break;
                }
            }
            if !done {
                /* Range is after all rules */
                new_ranges.push(range.clone());
            }
        }
        ranges = merge_ranges(&new_ranges);
    }
    let ans = ranges.iter().map(|range| range.start).min().unwrap();
    println!("{}", ans);
}
