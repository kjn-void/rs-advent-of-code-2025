use crate::days::Solution;

pub struct Day07 {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    start_col: usize,
}

impl Day07 {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            rows: 0,
            cols: 0,
            start_col: 0,
        }
    }
}

impl Solution for Day07 {
    fn set_input(&mut self, lines: &[String]) {
        self.grid.clear();

        // Copy lines exactly
        let mut max_cols = 0;
        for line in lines {
            max_cols = max_cols.max(line.len());
            self.grid.push(line.as_bytes().to_vec());
        }

        // Normalize width with spaces
        for row in &mut self.grid {
            if row.len() < max_cols {
                row.resize(max_cols, b' ');
            }
        }

        self.rows = self.grid.len();
        self.cols = max_cols;

        // Locate 'S' in first row
        self.start_col = self.grid[0]
            .iter()
            .position(|&c| c == b'S')
            .expect("Start position S not found");
    }

    // ------------------------------------------------------------
    // Part 1 — count split events
    // ------------------------------------------------------------
    fn part1(&mut self) -> String {
        let mut buf_a = vec![false; self.cols];
        let mut buf_b = vec![false; self.cols];

        let mut active = &mut buf_a;
        let mut next = &mut buf_b;

        active[self.start_col] = true;

        let mut split_count = 0;

        for r in 1..self.rows {
            let row = &self.grid[r];
            next.fill(false);

            for c in 0..self.cols {
                if !active[c] {
                    continue;
                }

                if row[c] == b'^' {
                    split_count += 1;
                    if c > 0 {
                        next[c - 1] = true;
                    }
                    if c + 1 < self.cols {
                        next[c + 1] = true;
                    }
                } else {
                    next[c] = true;
                }
            }

            std::mem::swap(&mut active, &mut next);
        }

        split_count.to_string()
    }

    // ------------------------------------------------------------
    // Part 2 — count all timelines
    // ------------------------------------------------------------
    fn part2(&mut self) -> String {
        let mut buf_a = vec![0i64; self.cols];
        let mut buf_b = vec![0i64; self.cols];

        let mut timelines = &mut buf_a;
        let mut next = &mut buf_b;

        timelines[self.start_col] = 1;

        for r in 1..self.rows {
            let row = &self.grid[r];
            next.fill(0);

            for c in 0..self.cols {
                let count = timelines[c];
                if count == 0 {
                    continue;
                }

                if row[c] == b'^' {
                    if c > 0 {
                        next[c - 1] += count;
                    }
                    if c + 1 < self.cols {
                        next[c + 1] += count;
                    }
                } else {
                    next[c] += count;
                }
            }

            std::mem::swap(&mut timelines, &mut next);
        }

        let total: i64 = timelines.iter().sum();
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day07::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "21");
    }

    #[test]
    fn part2_example() {
        let mut d = Day07::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "40");
    }
}