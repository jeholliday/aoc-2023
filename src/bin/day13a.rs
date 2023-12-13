use std::cmp::min;

enum Reflection {
    Vertical(usize),   // Reflection between columns with n columns to the left
    Horizontal(usize), // Reflection between rows with n rows above
}

fn solve_puzzle(puzzle: &str) -> Reflection {
    let rows: Vec<Vec<_>> = puzzle
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    for (i, j) in (0..rows.len())
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
    {
        let rows_to_cmp = min(i + 1, rows.len() - j);
        let mut matches = true;
        for k in 0..rows_to_cmp {
            let row_a = &rows[i - k];
            let row_b = &rows[j + k];
            if row_a != row_b {
                matches = false;
                break;
            }
        }
        if matches {
            return Reflection::Horizontal(j);
        }
    }

    let cols = (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (i, j) in (0..cols.len())
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
    {
        let cols_to_cmp = min(i + 1, cols.len() - j);
        let mut matches = true;
        for k in 0..cols_to_cmp {
            let col_a = &cols[i - k];
            let col_b = &cols[j + k];
            if col_a != col_b {
                matches = false;
                break;
            }
        }
        if matches {
            return Reflection::Vertical(j);
        }
    }

    panic!("No solution")
}

fn solve(input: &str) -> u64 {
    let puzzles: Vec<_> = input.trim().split("\n\n").collect();
    let reflections = puzzles
        .into_iter()
        .map(|p| solve_puzzle(p))
        .collect::<Vec<_>>();
    let ans: usize = reflections
        .into_iter()
        .map(|r| match r {
            Reflection::Vertical(n) => n,
            Reflection::Horizontal(n) => n * 100,
        })
        .sum();
    return ans as u64;
}

fn main() {
    let input = include_str!("../../inputs/day13.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#";
        assert_eq!(solve(input), 405);
    }
}
