use std::collections::HashMap;

fn solve(input: &str) -> u64 {
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
    let start_nodes: Vec<&str> = graph
        .keys()
        .filter(|node| node.chars().last().unwrap() == 'A')
        .copied()
        .collect();
    let mut end_nodes: Vec<Option<(&str, Vec<u32>)>> = start_nodes.iter().map(|_| None).collect();

    let mut nodes = start_nodes.clone();
    let mut steps: u32 = 0;
    loop {
        let mut next_nodes: Vec<&str> = Vec::new();
        let dir_index = steps as usize % dirs.len();
        let dir = dirs[dir_index];
        for node in nodes.iter() {
            let (left, right) = graph.get(node).unwrap();
            match dir {
                'L' => next_nodes.push(left),
                'R' => next_nodes.push(right),
                _ => panic!("Invalid direction"),
            }
        }
        nodes = next_nodes;
        steps += 1;
        let mut done = true;
        for (index, node) in nodes.iter().enumerate() {
            if node.chars().last().unwrap() == 'Z' {
                match end_nodes.get_mut(index).unwrap() {
                    Some((end_node, end_steps)) => {
                        assert!(end_node == node);
                        end_steps.push(steps);
                    }
                    None => {
                        end_nodes[index] = Some((node, vec![steps]));
                    }
                }
            }
            if end_nodes[index].is_none() {
                done = false;
            } else if end_nodes[index].as_ref().unwrap().1.len() < 10 {
                done = false;
            }
        }
        if done {
            break;
        }
    }

    let mut lines: Vec<u64> = Vec::new();
    for (index, _) in start_nodes.iter().enumerate() {
        let (_, end_steps) = end_nodes[index].as_ref().unwrap();
        let first = end_steps[0];
        let step = end_steps[1] - end_steps[0];
        for (j, step_count) in end_steps[1..].iter().enumerate() {
            assert!(step_count == &(first + (j as u32 + 1) * step));
        }
        lines.push(step as u64);
    }

    return lines
        .into_iter()
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();
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
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        assert_eq!(solve(input), 6);
    }
}
