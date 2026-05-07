use crate::days::Solution;

#[derive(Default)]
pub struct Day06 {
    grid: Vec<Vec<u8>>,
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

    // -----------------------------------------------------------
    // Number extractors
    // -----------------------------------------------------------

    // Takes a problem span, reads each row as one number, and returns the numbers for part 1.
    fn extract_row_numbers(&self, span: (usize, usize)) -> Vec<i64> {
        let (start, end) = span;
        let mut numbers = Vec::with_capacity(self.rows);

        for r in 0..self.rows - 1 {
            let s = self.grid[r][start..=end]
                .iter()
                .filter(|&&c| c != b' ')
                .map(|&c| c as char)
                .collect::<String>();
            numbers.push(s.parse::<i64>().unwrap());
        }

        numbers
    }

    // Takes a problem span, reads each column as one vertical number, and returns the numbers for part 2.
    fn extract_column_numbers(&self, span: (usize, usize)) -> Vec<i64> {
        let (start, end) = span;
        let mut numbers = Vec::with_capacity(end - start + 1);

        for c in start..=end {
            let mut s = String::new();
            for r in 0..self.rows - 1 {
                let ch = self.grid[r][c];
                if ch != b' ' {
                    s.push(ch as char);
                }
            }
            numbers.push(s.parse::<i64>().unwrap());
        }

        numbers
    }

    // -----------------------------------------------------------
    // Shared evaluation
    // -----------------------------------------------------------

    // Takes a number-extraction strategy, evaluates every worksheet problem, and returns their grand total.
    fn evaluate_blocks<F>(&self, extractor: F) -> i64
    where
        F: Fn(&Self, (usize, usize)) -> Vec<i64>,
    {
        let mut total = 0;

        for span in self.find_problem_spans() {
            let numbers = extractor(self, span);
            let operator = self.get_operator(span);
            total += eval_numbers(&numbers, operator);
        }

        total
    }
}

// Takes numbers and an operator byte, applies sum or product, and returns the problem's value.
fn eval_numbers(numbers: &[i64], operator: u8) -> i64 {
    if operator == b'+' {
        numbers.iter().sum()
    } else {
        numbers.iter().product()
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
    }

    // Evaluates row-oriented worksheet problems and returns their grand total.
    fn part1(&mut self) -> String {
        self.evaluate_blocks(Day06::extract_row_numbers).to_string()
    }

    // Evaluates column-oriented worksheet problems and returns their grand total.
    fn part2(&mut self) -> String {
        self.evaluate_blocks(Day06::extract_column_numbers)
            .to_string()
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
