use crate::days::Solution;
use rayon::prelude::*;
use std::f64;

#[derive(Clone)]
pub struct Machine {
    pub target_lights: Vec<i32>,
    pub target_joltage: Vec<i32>,
    pub buttons: Vec<Vec<usize>>,
}

#[derive(Default)]
pub struct Day10 {
    pub machines: Vec<Machine>,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }

    // ------------------------------------------------------------
    // Part 1: GF(2) solve using bitsets (optimized)
    // ------------------------------------------------------------
    // Takes one machine, solves its light toggles over GF(2), and returns the minimum button presses.
    fn fewest_light_presses(machine: &Machine) -> i32 {
        let light_count = machine.target_lights.len();
        let button_count = machine.buttons.len();
        if light_count == 0 || button_count == 0 {
            return 0;
        }

        let words = (button_count + 1).div_ceil(64);
        let mut matrix = vec![0u64; light_count * words];

        let bit = |row: usize, col: usize| -> usize { row * words + (col >> 6) };

        // RHS
        for light in 0..light_count {
            if machine.target_lights[light] != 0 {
                matrix[bit(light, button_count)] |= 1u64 << (button_count & 63);
            }
        }

        // Button matrix
        for (button, toggled_lights) in machine.buttons.iter().enumerate() {
            for &light in toggled_lights {
                if light < light_count {
                    matrix[bit(light, button)] |= 1u64 << (button & 63);
                }
            }
        }

        let mut pivot_row_for_button = vec![None; button_count];
        let mut row = 0;

        // Gaussian elimination (GF2)
        for (button, pivot) in pivot_row_for_button.iter_mut().enumerate() {
            if row >= light_count {
                break;
            }

            let word = button >> 6;
            let mask = 1u64 << (button & 63);

            let mut selected_row = None;
            for candidate_row in row..light_count {
                if (matrix[candidate_row * words + word] & mask) != 0 {
                    selected_row = Some(candidate_row);
                    break;
                }
            }

            let selected_row = match selected_row {
                Some(row) => row,
                None => continue,
            };

            if selected_row != row {
                let a = row * words;
                let b = selected_row * words;
                for word_offset in 0..words {
                    matrix.swap(a + word_offset, b + word_offset);
                }
            }

            *pivot = Some(row);

            for target_row in 0..light_count {
                if target_row != row {
                    let idx = target_row * words + word;
                    if (matrix[idx] & mask) != 0 {
                        let target_base = target_row * words;
                        let pivot_base = row * words;
                        for word_offset in 0..words {
                            matrix[target_base + word_offset] ^= matrix[pivot_base + word_offset];
                        }
                    }
                }
            }

            row += 1;
        }

        let free_buttons: Vec<usize> = (0..button_count)
            .filter(|&button| pivot_row_for_button[button].is_none())
            .collect();
        let mut best = i32::MAX;

        for mask in 0..(1u64 << free_buttons.len()) {
            let mut pressed = vec![0u64; words];

            for (index, &button) in free_buttons.iter().enumerate() {
                if (mask >> index) & 1 == 1 {
                    pressed[button >> 6] |= 1u64 << (button & 63);
                }
            }

            for button in (0..button_count).rev() {
                if let Some(pivot_row) = pivot_row_for_button[button] {
                    let mut value = (matrix[pivot_row * words + (button_count >> 6)]
                        >> (button_count & 63))
                        & 1;
                    for later_button in button + 1..button_count {
                        let coefficient = (matrix[pivot_row * words + (later_button >> 6)]
                            >> (later_button & 63))
                            & 1;
                        let later_pressed = (pressed[later_button >> 6] >> (later_button & 63)) & 1;
                        value ^= coefficient & later_pressed;
                    }
                    if value == 1 {
                        pressed[button >> 6] |= 1u64 << (button & 63);
                    }
                }
            }

            let sum: i32 = pressed.iter().map(|w| w.count_ones() as i32).sum();
            best = best.min(sum);
        }

        best
    }

    // ------------------------------------------------------------
    // Part 2: RREF + bounded integer DFS
    // ------------------------------------------------------------
    // Takes one machine, solves bounded integer joltage equations, and returns the minimum button presses.
    fn fewest_joltage_presses(machine: &Machine) -> i64 {
        let light_count = machine.target_joltage.len();
        let button_count = machine.buttons.len();
        if light_count == 0 || button_count == 0 {
            return 0;
        }

        let cols = button_count + 1;
        let mut matrix = vec![0.0f64; light_count * cols];
        for (i, &target) in machine.target_joltage.iter().enumerate() {
            matrix[i * cols + button_count] = target as f64;
        }
        for (j, btn) in machine.buttons.iter().enumerate() {
            for &i in btn {
                if i < light_count {
                    matrix[i * cols + j] = 1.0;
                }
            }
        }

        let mut pivot_row_for_col = vec![usize::MAX; button_count];
        let mut pivot_cols = Vec::with_capacity(light_count.min(button_count));
        let mut pivot_row = 0;

        for col in 0..button_count {
            if pivot_row >= light_count {
                break;
            }
            if let Some(selected_row) =
                (pivot_row..light_count).find(|&row| matrix[row * cols + col].abs() > 1e-9)
            {
                if selected_row != pivot_row {
                    for k in 0..=button_count {
                        matrix.swap(pivot_row * cols + k, selected_row * cols + k);
                    }
                }

                let row_base = pivot_row * cols;
                pivot_row_for_col[col] = pivot_row;
                pivot_cols.push(col);

                let divisor = matrix[row_base + col];
                for k in col..=button_count {
                    matrix[row_base + k] /= divisor;
                }
                for row in 0..light_count {
                    if row != pivot_row {
                        let base = row * cols;
                        let factor = matrix[base + col];
                        if factor.abs() > 1e-9 {
                            for k in col..=button_count {
                                matrix[base + k] -= factor * matrix[row_base + k];
                            }
                        }
                    }
                }
                pivot_row += 1;
            }
        }

        let mut free_cols: Vec<usize> = (0..button_count)
            .filter(|&col| pivot_row_for_col[col] == usize::MAX)
            .collect();

        let mut free_bounds: Vec<i32> = free_cols
            .iter()
            .map(|&col| {
                machine.buttons[col]
                    .iter()
                    .filter_map(|&idx| machine.target_joltage.get(idx).copied())
                    .min()
                    .unwrap_or(0)
            })
            .collect();

        let mut order: Vec<usize> = (0..free_cols.len()).collect();
        order.sort_unstable_by_key(|&i| free_bounds[i]);
        free_cols = order.iter().map(|&i| free_cols[i]).collect();
        free_bounds = order.iter().map(|&i| free_bounds[i]).collect();

        let free_len = free_cols.len();
        let pivot_count = pivot_cols.len();
        let mut pivot_rhs = vec![0.0; pivot_count];
        let mut pivot_free_coeff = vec![0.0; pivot_count * free_len];

        for (p, &col) in pivot_cols.iter().enumerate() {
            let row_base = pivot_row_for_col[col] * cols;
            pivot_rhs[p] = matrix[row_base + button_count];
            let coeff_base = p * free_len;
            for (free_index, &free_col) in free_cols.iter().enumerate() {
                pivot_free_coeff[coeff_base + free_index] = matrix[row_base + free_col];
            }
        }

        let mut best = i64::MAX;
        let mut free_values = vec![0i32; free_len];

        // Recurses over free button press counts, derives pivot counts, and updates the best feasible total.
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
// Takes a parenthesized/comma-separated list, parses indices, and returns the button wiring targets.
fn parse_list(s: &str) -> Vec<usize> {
    s.trim_matches(|c| c == '(' || c == ')' || c == '{' || c == '}')
        .split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect()
}

// Takes one machine description line, parses lights, buttons, and joltage targets, and returns a Machine.
fn parse_machine(line: &str) -> Machine {
    let mut parts = line.split_whitespace();

    let lights_str = parts.next().unwrap();
    let target_lights = lights_str
        .trim_matches(&['[', ']'][..])
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();

    let mut buttons = Vec::new();
    let mut target_joltage = Vec::new();

    for p in parts {
        if p.starts_with('(') {
            buttons.push(parse_list(p));
        } else if p.starts_with('{') {
            target_joltage = p
                .trim_matches(&['{', '}'][..])
                .split(',')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();
        }
    }

    Machine {
        target_lights,
        target_joltage,
        buttons,
    }
}

// ------------------------------------------------------------
// Trait implementation
// ------------------------------------------------------------
impl Solution for Day10 {
    // Takes machine description lines, parses each non-empty line, and stores all machines.
    fn set_input(&mut self, lines: &[String]) {
        self.machines.clear();
        for line in lines {
            if !line.trim().is_empty() {
                self.machines.push(parse_machine(line));
            }
        }
    }

    // Solves all machines' light states in parallel and returns the summed minimum button presses.
    fn part1(&mut self) -> String {
        self.machines
            .par_iter()
            .map(|machine| Self::fewest_light_presses(machine) as i64)
            .sum::<i64>()
            .to_string()
    }

    // Solves all machines' joltage targets in parallel and returns the summed minimum button presses.
    fn part2(&mut self) -> String {
        self.machines
            .par_iter()
            .map(Self::fewest_joltage_presses)
            .sum::<i64>()
            .to_string()
    }
}
