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

        let bit = |row: usize, col: usize| -> usize { row * words + (col >> 6) };

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
    // Part 2: RREF + bounded integer DFS
    // ------------------------------------------------------------
    fn solve_joltage(machine: &Machine) -> i64 {
        let n = machine.target_joltage.len();
        let m = machine.buttons.len();
        if n == 0 || m == 0 {
            return 0;
        }

        let cols = m + 1;
        let mut mat = vec![0.0f64; n * cols];
        for (i, &target) in machine.target_joltage.iter().enumerate() {
            mat[i * cols + m] = target as f64;
        }
        for (j, btn) in machine.buttons.iter().enumerate() {
            for &i in btn {
                if i < n {
                    mat[i * cols + j] = 1.0;
                }
            }
        }

        let mut pivot_row_for_col = vec![usize::MAX; m];
        let mut pivot_cols = Vec::with_capacity(n.min(m));
        let mut r = 0;

        for c in 0..m {
            if r >= n {
                break;
            }
            if let Some(sel) = (r..n).find(|&i| mat[i * cols + c].abs() > 1e-9) {
                if sel != r {
                    for k in 0..=m {
                        mat.swap(r * cols + k, sel * cols + k);
                    }
                }

                let row_base = r * cols;
                pivot_row_for_col[c] = r;
                pivot_cols.push(c);

                let div = mat[row_base + c];
                for k in c..=m {
                    mat[row_base + k] /= div;
                }
                for i in 0..n {
                    if i != r {
                        let base = i * cols;
                        let f = mat[base + c];
                        if f.abs() > 1e-9 {
                            for k in c..=m {
                                mat[base + k] -= f * mat[row_base + k];
                            }
                        }
                    }
                }
                r += 1;
            }
        }

        let mut free: Vec<usize> = (0..m)
            .filter(|&c| pivot_row_for_col[c] == usize::MAX)
            .collect();

        let mut free_bounds: Vec<i32> = free
            .iter()
            .map(|&col| {
                machine.buttons[col]
                    .iter()
                    .filter_map(|&idx| machine.target_joltage.get(idx).copied())
                    .min()
                    .unwrap_or(0)
            })
            .collect();

        let mut order: Vec<usize> = (0..free.len()).collect();
        order.sort_unstable_by_key(|&i| free_bounds[i]);
        free = order.iter().map(|&i| free[i]).collect();
        free_bounds = order.iter().map(|&i| free_bounds[i]).collect();

        let free_len = free.len();
        let pivot_count = pivot_cols.len();
        let mut pivot_rhs = vec![0.0; pivot_count];
        let mut pivot_free_coeff = vec![0.0; pivot_count * free_len];

        for (p, &col) in pivot_cols.iter().enumerate() {
            let row_base = pivot_row_for_col[col] * cols;
            pivot_rhs[p] = mat[row_base + m];
            let coeff_base = p * free_len;
            for (f, &free_col) in free.iter().enumerate() {
                pivot_free_coeff[coeff_base + f] = mat[row_base + free_col];
            }
        }

        let mut best = i64::MAX;
        let mut free_values = vec![0i32; free_len];

        fn dfs(
            idx: usize,
            free_bounds: &[i32],
            free_values: &mut [i32],
            pivot_rhs: &[f64],
            pivot_free_coeff: &[f64],
            cur: i64,
            best: &mut i64,
        ) {
            if cur >= *best {
                return;
            }
            if idx == free_bounds.len() {
                let mut total = cur;
                let free_len = free_bounds.len();

                for (p, &rhs) in pivot_rhs.iter().enumerate() {
                    let mut v = rhs;
                    let coeff_base = p * free_len;
                    for (f, &x) in free_values.iter().enumerate() {
                        let coeff = pivot_free_coeff[coeff_base + f];
                        if coeff.abs() > 1e-9 {
                            v -= coeff * f64::from(x);
                        }
                    }

                    let iv = v.round();
                    if (v - iv).abs() > 1e-6 || iv < 0.0 {
                        return;
                    }
                    total += iv as i64;
                    if total >= *best {
                        return;
                    }
                }

                *best = (*best).min(total);
                return;
            }

            for v in 0..=free_bounds[idx] {
                free_values[idx] = v;
                dfs(
                    idx + 1,
                    free_bounds,
                    free_values,
                    pivot_rhs,
                    pivot_free_coeff,
                    cur + v as i64,
                    best,
                );
                if cur + v as i64 >= *best {
                    break;
                }
            }
        }

        dfs(
            0,
            &free_bounds,
            &mut free_values,
            &pivot_rhs,
            &pivot_free_coeff,
            0,
            &mut best,
        );
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
