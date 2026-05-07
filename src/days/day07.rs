use crate::days::Solution;

#[derive(Default)]
pub struct Day07 {
    manifold: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    start_col: usize,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day07 {
    // Takes the manifold diagram, pads rows to equal width, and records the starting beam column.
    fn set_input(&mut self, lines: &[String]) {
        self.manifold.clear();

        // Copy lines exactly
        let mut max_cols = 0;
        for line in lines {
            max_cols = max_cols.max(line.len());
            self.manifold.push(line.as_bytes().to_vec());
        }

        // Normalize width with spaces
        for row in &mut self.manifold {
            if row.len() < max_cols {
                row.resize(max_cols, b' ');
            }
        }

        self.rows = self.manifold.len();
        self.cols = max_cols;

        // Locate 'S' in first row
        self.start_col = self.manifold[0]
            .iter()
            .position(|&c| c == b'S')
            .expect("Start position S not found");
    }

    // Propagates reachable beam columns through the manifold and returns the number of splitter hits.
    fn part1(&mut self) -> String {
        let mut buf_a = vec![false; self.cols];
        let mut buf_b = vec![false; self.cols];

        let mut active_beams = &mut buf_a;
        let mut next_beams = &mut buf_b;

        active_beams[self.start_col] = true;

        let mut split_count = 0;

        for r in 1..self.rows {
            let row = &self.manifold[r];
            next_beams.fill(false);

            for c in 0..self.cols {
                if !active_beams[c] {
                    continue;
                }

                if row[c] == b'^' {
                    split_count += 1;
                    if c > 0 {
                        next_beams[c - 1] = true;
                    }
                    if c + 1 < self.cols {
                        next_beams[c + 1] = true;
                    }
                } else {
                    next_beams[c] = true;
                }
            }

            std::mem::swap(&mut active_beams, &mut next_beams);
        }

        split_count.to_string()
    }

    // Propagates beam path counts through splitters and returns the number of exiting beam timelines.
    fn part2(&mut self) -> String {
        let mut buf_a = vec![0i64; self.cols];
        let mut buf_b = vec![0i64; self.cols];

        let mut beam_counts = &mut buf_a;
        let mut next_counts = &mut buf_b;

        beam_counts[self.start_col] = 1;

        for r in 1..self.rows {
            let row = &self.manifold[r];
            next_counts.fill(0);

            for c in 0..self.cols {
                let count = beam_counts[c];
                if count == 0 {
                    continue;
                }

                if row[c] == b'^' {
                    if c > 0 {
                        next_counts[c - 1] += count;
                    }
                    if c + 1 < self.cols {
                        next_counts[c + 1] += count;
                    }
                } else {
                    next_counts[c] += count;
                }
            }

            std::mem::swap(&mut beam_counts, &mut next_counts);
        }

        let total: i64 = beam_counts.iter().sum();
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
