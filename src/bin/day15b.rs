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
    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::new(); 256];
    let steps: Vec<_> = input.trim().split(',').collect();
    for step in steps {
        if step.contains('-') {
            let label = step.split('-').next().unwrap();
            let box_num = hash(label);
            let b = &mut boxes[box_num as usize];
            let mut found_index = None;
            for (i, (k, _)) in b.iter().enumerate() {
                if k == label {
                    found_index = Some(i);
                    break;
                }
            }
            if let Some(i) = found_index {
                b.remove(i);
            }
        } else if step.contains('=') {
            let label = step.split('=').next().unwrap();
            let num = step.split('=').last().unwrap().parse::<u8>().unwrap();
            let box_num = hash(label);
            let b = &mut boxes[box_num as usize];
            let mut found_index = None;
            for (i, (k, _)) in b.iter().enumerate() {
                if k == label {
                    found_index = Some(i);
                    break;
                }
            }
            if let Some(i) = found_index {
                b[i] = (label.into(), num);
            } else {
                b.push((label.into(), num));
            }
        } else {
            panic!("Invalid step");
        }

        /*println!("After \"{}\":", step);
        for (i, b) in boxes.iter().enumerate() {
            if !b.is_empty() {
                print!("Box {}:", i);
                for (k, v) in b {
                    print!(" [{} {}]", k, v);
                }
                println!();
            }
        }
        println!();*/
    }
    boxes.iter().enumerate().map(|(i, the_box)| {
        the_box.iter().enumerate().map(|(j, (_, num))| {
            let a = i + 1;
            let b = j + 1;
            let c = num;
            a as u64 * b as u64 * *c as u64
        }).sum::<u64>()
    }).sum()
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
        assert_eq!(solve(input), 145);
    }
}
