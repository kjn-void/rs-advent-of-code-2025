use crate::days::Solution;

#[derive(Default)]
pub struct Day06 {
    grid: Vec<Vec<u8>>,
    spans: Vec<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    // -----------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------

    // Scans columns for blank separators, groups each worksheet problem span, and returns inclusive column bounds.
    fn find_problem_spans(&self) -> Vec<(usize, usize)> {
        let mut is_blank = vec![true; self.cols];

        for (c, blank) in is_blank.iter_mut().enumerate() {
            for r in 0..self.rows {
                if self.grid[r][c] != b' ' {
                    *blank = false;
                    break;
                }
            }
        }

        let mut spans = Vec::new();
        let mut in_block = false;
        let mut start = 0;

        for (c, &blank) in is_blank.iter().enumerate() {
            if !blank {
                if !in_block {
                    in_block = true;
                    start = c;
                }
            } else if in_block {
                spans.push((start, c - 1));
                in_block = false;
            }
        }

        if in_block {
            spans.push((start, self.cols - 1));
        }

        spans
    }

    // Takes a problem span, finds its bottom-row operator, and returns '+' or '*'.
    fn get_operator(&self, span: (usize, usize)) -> u8 {
        let (start, end) = span;
        let row = &self.grid[self.rows - 1][start..=end];
        for &ch in row {
            if ch == b'+' || ch == b'*' {
                return ch;
            }
        }
        b'*' // AoC guarantees this won't happen
    }

    // Reads each worksheet row directly as decimal digits and returns the part 1 total.
    fn evaluate_rows(&self) -> i64 {
        let mut total = 0;
        for &span @ (start, end) in &self.spans {
            let operator = self.get_operator(span);
            let mut value = if operator == b'+' { 0 } else { 1 };
            for row in &self.grid[..self.rows - 1] {
                let number = row[start..=end]
                    .iter()
                    .filter(|&&byte| byte != b' ')
                    .fold(0i64, |number, &byte| number * 10 + i64::from(byte - b'0'));
                if operator == b'+' {
                    value += number;
                } else {
                    value *= number;
                }
            }
            total += value;
        }
        total
    }

    // Reads each worksheet column directly as decimal digits and returns the part 2 total.
    fn evaluate_columns(&self) -> i64 {
        let mut total = 0;
        for &span @ (start, end) in &self.spans {
            let operator = self.get_operator(span);
            let mut value = if operator == b'+' { 0 } else { 1 };
            for col in start..=end {
                let mut number = 0i64;
                for row in &self.grid[..self.rows - 1] {
                    let byte = row[col];
                    if byte != b' ' {
                        number = number * 10 + i64::from(byte - b'0');
                    }
                }
                if operator == b'+' {
                    value += number;
                } else {
                    value *= number;
                }
            }
            total += value;
        }
        total
    }
}

impl Solution for Day06 {
    // Takes the worksheet rows, pads them to equal width, and stores the normalized grid.
    fn set_input(&mut self, lines: &[String]) {
        self.grid.clear();

        let mut max_cols = 0;
        for line in lines {
            max_cols = max_cols.max(line.len());
            self.grid.push(line.as_bytes().to_vec());
        }

        for row in &mut self.grid {
            if row.len() < max_cols {
                row.resize(max_cols, b' ');
            }
        }

        self.rows = self.grid.len();
        self.cols = max_cols;
        self.spans = self.find_problem_spans();
    }

    // Evaluates row-oriented worksheet problems and returns their grand total.
    fn part1(&mut self) -> String {
        self.evaluate_rows().to_string()
    }

    // Evaluates column-oriented worksheet problems and returns their grand total.
    fn part2(&mut self) -> String {
        self.evaluate_columns().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day06::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "4277556");
    }

    #[test]
    fn part2_example() {
        let mut d = Day06::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "3263827");
    }
}
