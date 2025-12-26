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
    // Part 1: GF(2) solve using bitsets (optimized)
    // ------------------------------------------------------------
    fn solve_lights(machine: &Machine) -> i32 {
        let n = machine.target_lights.len();
        let m = machine.buttons.len();
        if n == 0 || m == 0 {
            return 0;
        }

        let words = (m + 1 + 63) / 64;
        let mut mat = vec![0u64; n * words];

        let bit = |row: usize, col: usize| -> usize {
            row * words + (col >> 6)
        };

        // RHS
        for i in 0..n {
            if machine.target_lights[i] != 0 {
                mat[bit(i, m)] |= 1u64 << (m & 63);
            }
        }

        // Button matrix
        for (j, btn) in machine.buttons.iter().enumerate() {
            for &i in btn {
                if i < n {
                    mat[bit(i, j)] |= 1u64 << (j & 63);
                }
            }
        }

        let mut pivot_col = vec![None; m];
        let mut row = 0;

        // Gaussian elimination (GF2)
        for col in 0..m {
            if row >= n {
                break;
            }

            let word = col >> 6;
            let mask = 1u64 << (col & 63);

            let mut rsel = None;
            for r in row..n {
                if (mat[r * words + word] & mask) != 0 {
                    rsel = Some(r);
                    break;
                }
            }

            let rsel = match rsel {
                Some(r) => r,
                None => continue,
            };

            if rsel != row {
                let a = row * words;
                let b = rsel * words;
                for k in 0..words {
                    mat.swap(a + k, b + k);
                }
            }

            pivot_col[col] = Some(row);

            for r in 0..n {
                if r != row {
                    let idx = r * words + word;
                    if (mat[idx] & mask) != 0 {
                        let ra = r * words;
                        let rb = row * words;
                        for k in 0..words {
                            mat[ra + k] ^= mat[rb + k];
                        }
                    }
                }
            }

            row += 1;
        }

        let free: Vec<usize> = (0..m).filter(|&c| pivot_col[c].is_none()).collect();
        let mut best = i32::MAX;

        for mask in 0..(1u64 << free.len()) {
            let mut x = vec![0u64; words];

            for (i, &c) in free.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    x[c >> 6] |= 1u64 << (c & 63);
                }
            }

            for c in (0..m).rev() {
                if let Some(rp) = pivot_col[c] {
                    let mut v = (mat[rp * words + (m >> 6)] >> (m & 63)) & 1;
                    for k in c + 1..m {
                        let a = (mat[rp * words + (k >> 6)] >> (k & 63)) & 1;
                        let b = (x[k >> 6] >> (k & 63)) & 1;
                        v ^= a & b;
                    }
                    if v == 1 {
                        x[c >> 6] |= 1u64 << (c & 63);
                    }
                }
            }

            let sum: i32 = x.iter().map(|w| w.count_ones() as i32).sum();
            best = best.min(sum);
        }

        best
    }

    // ------------------------------------------------------------
    // Part 2: RREF + bounded integer DFS (unchanged)
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
            if !line.trim().is_empty() {
                self.machines.push(parse_machine(line));
            }
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