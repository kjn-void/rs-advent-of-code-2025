use crate::days::Solution;

pub struct Day06 {
    grid: Vec<Vec<u8>>,
    r: usize,
    c: usize,
}

impl Day06 {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            r: 0,
            c: 0,
        }
    }

    // -----------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------

    fn find_blocks(&self) -> Vec<(usize, usize)> {
        let mut is_blank = vec![true; self.c];

        for c in 0..self.c {
            for r in 0..self.r {
                if self.grid[r][c] != b' ' {
                    is_blank[c] = false;
                    break;
                }
            }
        }

        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut start = 0;

        for c in 0..self.c {
            if !is_blank[c] {
                if !in_block {
                    in_block = true;
                    start = c;
                }
            } else if in_block {
                blocks.push((start, c - 1));
                in_block = false;
            }
        }

        if in_block {
            blocks.push((start, self.c - 1));
        }

        blocks
    }

    fn get_operator(&self, b: (usize, usize)) -> u8 {
        let (start, end) = b;
        let row = &self.grid[self.r - 1][start..=end];
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

    fn extract_numbers_part1(&self, b: (usize, usize)) -> Vec<i64> {
        let (start, end) = b;
        let mut nums = Vec::with_capacity(self.r);

        for r in 0..self.r - 1 {
            let s = self.grid[r][start..=end]
                .iter()
                .filter(|&&c| c != b' ')
                .map(|&c| c as char)
                .collect::<String>();
            nums.push(s.parse::<i64>().unwrap());
        }

        nums
    }

    fn extract_numbers_part2(&self, b: (usize, usize)) -> Vec<i64> {
        let (start, end) = b;
        let mut nums = Vec::with_capacity(end - start + 1);

        for c in start..=end {
            let mut s = String::new();
            for r in 0..self.r - 1 {
                let ch = self.grid[r][c];
                if ch != b' ' {
                    s.push(ch as char);
                }
            }
            nums.push(s.parse::<i64>().unwrap());
        }

        nums
    }

    // -----------------------------------------------------------
    // Shared evaluation
    // -----------------------------------------------------------

    fn evaluate_blocks<F>(&self, extractor: F) -> i64
    where
        F: Fn(&Self, (usize, usize)) -> Vec<i64>,
    {
        let mut total = 0;

        for b in self.find_blocks() {
            let nums = extractor(self, b);
            let op = self.get_operator(b);
            total += eval_numbers(&nums, op);
        }

        total
    }
}

fn eval_numbers(nums: &[i64], op: u8) -> i64 {
    if op == b'+' {
        nums.iter().sum()
    } else {
        nums.iter().product()
    }
}

impl Solution for Day06 {
    fn set_input(&mut self, lines: &[String]) {
        self.grid.clear();

        let mut max_c = 0;
        for line in lines {
            max_c = max_c.max(line.len());
            self.grid.push(line.as_bytes().to_vec());
        }

        for row in &mut self.grid {
            if row.len() < max_c {
                row.resize(max_c, b' ');
            }
        }

        self.r = self.grid.len();
        self.c = max_c;
    }

    fn part1(&mut self) -> String {
        self.evaluate_blocks(Day06::extract_numbers_part1)
            .to_string()
    }

    fn part2(&mut self) -> String {
        self.evaluate_blocks(Day06::extract_numbers_part2)
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