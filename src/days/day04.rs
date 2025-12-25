use crate::days::Solution;

pub struct Day04 {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Day04 {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    // 8 directions
    const DIRS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    fn make_bool_grid(&self) -> Vec<Vec<bool>> {
        let mut out = vec![vec![false; self.cols]; self.rows];
        for r in 0..self.rows {
            for c in 0..self.cols {
                out[r][c] = self.grid[r][c] == b'@';
            }
        }
        out
    }

    fn compute_degrees(&self, on: &[Vec<bool>]) -> Vec<Vec<i32>> {
        let mut deg = vec![vec![0; self.cols]; self.rows];

        for r in 0..self.rows {
            for c in 0..self.cols {
                if !on[r][c] {
                    continue;
                }
                let mut cnt = 0;
                for (dr, dc) in Self::DIRS {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0
                        && nr < self.rows as isize
                        && nc >= 0
                        && nc < self.cols as isize
                        && on[nr as usize][nc as usize]
                    {
                        cnt += 1;
                    }
                }
                deg[r][c] = cnt;
            }
        }
        deg
    }

    fn count_adjacent(&self, r: usize, c: usize) -> i32 {
        let mut count = 0;
        for (dr, dc) in Self::DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0
                && nr < self.rows as isize
                && nc >= 0
                && nc < self.cols as isize
                && self.grid[nr as usize][nc as usize] == b'@'
            {
                count += 1;
            }
        }
        count
    }
}

impl Solution for Day04 {
    fn set_input(&mut self, lines: &[String]) {
        self.grid.clear();

        for line in lines {
            self.grid.push(line.as_bytes().to_vec());
        }

        self.rows = self.grid.len();
        self.cols = if self.rows > 0 { self.grid[0].len() } else { 0 };
    }

    fn part1(&mut self) -> String {
        if self.rows == 0 || self.cols == 0 {
            return "0".to_string();
        }

        let mut total = 0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == b'@' && self.count_adjacent(r, c) < 4 {
                    total += 1;
                }
            }
        }

        total.to_string()
    }

    fn part2(&mut self) -> String {
        if self.rows == 0 || self.cols == 0 {
            return "0".to_string();
        }

        let mut on = self.make_bool_grid();
        let mut deg = self.compute_degrees(&on);

        let mut queue: Vec<(usize, usize)> = Vec::new();

        for r in 0..self.rows {
            for c in 0..self.cols {
                if on[r][c] && deg[r][c] < 4 {
                    queue.push((r, c));
                }
            }
        }

        let mut removed = 0;
        let mut qp = 0;

        while qp < queue.len() {
            let (r, c) = queue[qp];
            qp += 1;

            if !on[r][c] {
                continue;
            }

            on[r][c] = false;
            removed += 1;

            for (dr, dc) in Self::DIRS {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0
                    || nr >= self.rows as isize
                    || nc < 0
                    || nc >= self.cols as isize
                {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;

                if !on[nr][nc] {
                    continue;
                }

                deg[nr][nc] -= 1;
                if deg[nr][nc] == 3 {
                    queue.push((nr, nc));
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