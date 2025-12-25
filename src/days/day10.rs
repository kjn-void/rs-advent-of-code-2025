use crate::days::Solution;
use rayon::prelude::*;
use std::f64;

#[derive(Clone)]
pub struct Machine {
    pub target_lights: Vec<i32>,
    pub target_joltage: Vec<i32>,
    pub buttons: Vec<Vec<usize>>,
}

pub struct Day10 {
    pub machines: Vec<Machine>,
}

impl Day10 {
    pub fn new() -> Self {
        Self {
            machines: Vec::new(),
        }
    }

    // ------------------------------------------------------------
    // Part 1: Solve over GF(2), minimize Hamming weight
    // ------------------------------------------------------------
    fn solve_lights(machine: &Machine) -> i32 {
        let n = machine.target_lights.len();
        let m = machine.buttons.len();
        if n == 0 || m == 0 {
            return 0;
        }

        // Augmented matrix N x (M+1)
        let mut mat = vec![vec![0i32; m + 1]; n];
        for i in 0..n {
            mat[i][m] = machine.target_lights[i];
        }
        for (j, btn) in machine.buttons.iter().enumerate() {
            for &i in btn {
                if i < n {
                    mat[i][j] = 1;
                }
            }
        }

        let mut pivot_col = vec![None; m];
        let mut r = 0;

        // Gaussian elimination (GF2)
        for c in 0..m {
            if r >= n {
                break;
            }
            if let Some(sel) = (r..n).find(|&i| mat[i][c] == 1) {
                mat.swap(r, sel);
                pivot_col[c] = Some(r);
                for i in 0..n {
                    if i != r && mat[i][c] == 1 {
                        for k in c..=m {
                            mat[i][k] ^= mat[r][k];
                        }
                    }
                }
                r += 1;
            }
        }

        for i in r..n {
            if mat[i][m] == 1 {
                return i32::MAX;
            }
        }

        let free: Vec<usize> = (0..m).filter(|&c| pivot_col[c].is_none()).collect();
        let mut best = i32::MAX;

        for mask in 0..(1 << free.len()) {
            let mut x = vec![0; m];

            for (i, &c) in free.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    x[c] = 1;
                }
            }

            for c in (0..m).rev() {
                if let Some(rp) = pivot_col[c] {
                    let mut v = mat[rp][m];
                    for k in c + 1..m {
                        v ^= mat[rp][k] & x[k];
                    }
                    x[c] = v;
                }
            }

            let sum: i32 = x.iter().sum();
            best = best.min(sum);
        }

        best
    }

    // ------------------------------------------------------------
    // Part 2: RREF + bounded integer search
    // ------------------------------------------------------------
    fn solve_joltage(machine: &Machine) -> i64 {
        let n = machine.target_joltage.len();
        let m = machine.buttons.len();
        if n == 0 || m == 0 {
            return 0;
        }

        let mut mat = vec![vec![0.0f64; m + 1]; n];
        for i in 0..n {
            mat[i][m] = machine.target_joltage[i] as f64;
        }
        for (j, btn) in machine.buttons.iter().enumerate() {
            for &i in btn {
                if i < n {
                    mat[i][j] = 1.0;
                }
            }
        }

        // RREF
        let mut pivot_col = vec![None; m];
        let mut r = 0;

        for c in 0..m {
            if r >= n {
                break;
            }
            if let Some(sel) = (r..n).find(|&i| mat[i][c].abs() > 1e-9) {
                mat.swap(r, sel);
                let div = mat[r][c];
                for k in c..=m {
                    mat[r][k] /= div;
                }
                for i in 0..n {
                    if i != r {
                        let f = mat[i][c];
                        if f.abs() > 1e-9 {
                            for k in c..=m {
                                mat[i][k] -= f * mat[r][k];
                            }
                        }
                    }
                }
                pivot_col[c] = Some(r);
                r += 1;
            }
        }

        let free: Vec<usize> = (0..m).filter(|&c| pivot_col[c].is_none()).collect();
        let bound = machine.target_joltage.iter().max().copied().unwrap_or(0) + 1;

        let mut best = i64::MAX;
        let mut x = vec![0.0; m];

        fn dfs(
            idx: usize,
            free: &[usize],
            mat: &[Vec<f64>],
            pivot_col: &[Option<usize>],
            x: &mut [f64],
            cur: i64,
            best: &mut i64,
            bound: i32,
        ) {
            if cur >= *best {
                return;
            }
            if idx == free.len() {
                let mut total = cur;
                for c in 0..x.len() {
                    if let Some(r) = pivot_col[c] {
                        let mut v = mat[r][x.len()];
                        for k in c + 1..x.len() {
                            v -= mat[r][k] * x[k];
                        }
                        let iv = v.round();
                        if (v - iv).abs() > 1e-6 || iv < 0.0 {
                            return;
                        }
                        total += iv as i64;
                    }
                }
                *best = (*best).min(total);
                return;
            }

            let c = free[idx];
            for v in 0..=bound {
                x[c] = v as f64;
                dfs(
                    idx + 1,
                    free,
                    mat,
                    pivot_col,
                    x,
                    cur + v as i64,
                    best,
                    bound,
                );
                if cur + v as i64 >= *best {
                    break;
                }
            }
        }

        dfs(0, &free, &mat, &pivot_col, &mut x, 0, &mut best, bound);
        best
    }
}

// ------------------------------------------------------------
// Parsing
// ------------------------------------------------------------
fn parse_list(s: &str) -> Vec<usize> {
    s.trim_matches(|c| c == '(' || c == ')' || c == '{' || c == '}')
        .split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect()
}

fn parse_machine(line: &str) -> Machine {
    let mut parts = line.split_whitespace();

    let lights_str = parts.next().unwrap();
    let lights = lights_str
        .trim_matches(&['[', ']'][..])
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();

    let mut buttons = Vec::new();
    let mut joltage = Vec::new();

    for p in parts {
        if p.starts_with('(') {
            buttons.push(parse_list(p));
        } else if p.starts_with('{') {
            joltage = p
                .trim_matches(&['{', '}'][..])
                .split(',')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();
        }
    }

    Machine {
        target_lights: lights,
        target_joltage: joltage,
        buttons,
    }
}

// ------------------------------------------------------------
// Trait implementation
// ------------------------------------------------------------
impl Solution for Day10 {
    fn set_input(&mut self, lines: &[String]) {
        self.machines.clear();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            self.machines.push(parse_machine(line));
        }
    }

    fn part1(&mut self) -> String {
        self.machines
            .par_iter()
            .map(|m| Self::solve_lights(m) as i64)
            .sum::<i64>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        self.machines
            .par_iter()
            .map(|m| Self::solve_joltage(m))
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    const DAY10_EXAMPLE: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    fn split_lines(s: &str) -> Vec<String> {
        s.lines()
            .map(|l| l.trim_end())
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day10::new();
        d.set_input(&split_lines(DAY10_EXAMPLE));

        let got = d.part1();
        let want = "7";

        assert_eq!(got, want, "Part 1 example failed");
    }

    #[test]
    fn part2_example() {
        let mut d = Day10::new();
        d.set_input(&split_lines(DAY10_EXAMPLE));

        let got = d.part2();
        let want = "33";

        assert_eq!(got, want, "Part 2 example failed");
    }
}