use crate::days::Solution;

#[derive(Default)]
pub struct Day04 {
    grid: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    // 8 directions
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // Converts the input map into a boolean roll-presence grid used by the removal simulation.
    fn paper_roll_grid(&self) -> Vec<bool> {
        self.grid.iter().map(|&cell| cell == b'@').collect()
    }

    // Takes a roll-presence grid, counts adjacent rolls for each occupied cell, and returns those counts.
    fn count_neighbor_rolls(&self, has_roll: &[bool]) -> Vec<u8> {
        let mut neighbor_counts = vec![0; self.grid.len()];

        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = row * self.cols + col;
                if !has_roll[index] {
                    continue;
                }
                let mut count = 0;
                for (dr, dc) in Self::DIRS {
                    let neighbor_row = row as isize + dr;
                    let neighbor_col = col as isize + dc;
                    if neighbor_row >= 0
                        && neighbor_row < self.rows as isize
                        && neighbor_col >= 0
                        && neighbor_col < self.cols as isize
                        && has_roll[neighbor_row as usize * self.cols + neighbor_col as usize]
                    {
                        count += 1;
                    }
                }
                neighbor_counts[index] = count;
            }
        }
        neighbor_counts
    }

    // Takes a grid coordinate and returns how many of its eight neighboring cells contain rolls.
    fn count_adjacent_rolls(&self, row: usize, col: usize) -> i32 {
        let mut count = 0;
        for (dr, dc) in Self::DIRS {
            let neighbor_row = row as isize + dr;
            let neighbor_col = col as isize + dc;
            if neighbor_row >= 0
                && neighbor_row < self.rows as isize
                && neighbor_col >= 0
                && neighbor_col < self.cols as isize
                && self.grid[neighbor_row as usize * self.cols + neighbor_col as usize] == b'@'
            {
                count += 1;
            }
        }
        count
    }
}

impl Solution for Day04 {
    // Takes the paper-roll map, stores it as bytes, and records its dimensions.
    fn set_input(&mut self, lines: &[String]) {
        self.grid.clear();

        self.rows = lines.len();
        self.cols = lines.first().map_or(0, String::len);
        self.grid.reserve(self.rows * self.cols);
        for line in lines {
            self.grid.extend_from_slice(line.as_bytes());
        }
    }

    // Counts rolls immediately accessible under the adjacency rule and returns that count.
    fn part1(&mut self) -> String {
        if self.rows == 0 || self.cols == 0 {
            return "0".to_string();
        }

        let mut total = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.grid[row * self.cols + col] == b'@'
                    && self.count_adjacent_rolls(row, col) < 4
                {
                    total += 1;
                }
            }
        }

        total.to_string()
    }

    // Repeatedly removes accessible rolls, updates neighbor counts, and returns the total removed.
    fn part2(&mut self) -> String {
        if self.rows == 0 || self.cols == 0 {
            return "0".to_string();
        }

        let mut has_roll = self.paper_roll_grid();
        let mut neighbor_counts = self.count_neighbor_rolls(&has_roll);

        let mut queue: Vec<(usize, usize)> = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = row * self.cols + col;
                if has_roll[index] && neighbor_counts[index] < 4 {
                    queue.push((row, col));
                }
            }
        }

        let mut removed = 0;
        let mut queue_pos = 0;

        while queue_pos < queue.len() {
            let (row, col) = queue[queue_pos];
            queue_pos += 1;
            let index = row * self.cols + col;

            if !has_roll[index] {
                continue;
            }

            has_roll[index] = false;
            removed += 1;

            for (dr, dc) in Self::DIRS {
                let neighbor_row = row as isize + dr;
                let neighbor_col = col as isize + dc;
                if neighbor_row < 0
                    || neighbor_row >= self.rows as isize
                    || neighbor_col < 0
                    || neighbor_col >= self.cols as isize
                {
                    continue;
                }
                let neighbor_row = neighbor_row as usize;
                let neighbor_col = neighbor_col as usize;
                let neighbor_index = neighbor_row * self.cols + neighbor_col;

                if !has_roll[neighbor_index] {
                    continue;
                }

                neighbor_counts[neighbor_index] -= 1;
                if neighbor_counts[neighbor_index] == 3 {
                    queue.push((neighbor_row, neighbor_col));
                }
            }
        }

        removed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day04::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "13");
    }

    #[test]
    fn part2_example() {
        let mut d = Day04::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "43");
    }
}
