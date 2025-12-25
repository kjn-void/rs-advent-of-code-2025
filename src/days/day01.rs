use crate::days::Solution;

pub struct Day01 {
    // Signed deltas: Rn => +n, Ln => -n
    moves: Vec<i32>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    #[inline]
    fn mod100(n: i32) -> i32 {
        let mut v = n % 100;
        if v < 0 {
            v += 100;
        }
        v
    }
}

impl Solution for Day01 {
    fn set_input(&mut self, lines: &[String]) {
        self.moves.clear();

        for line in lines {
            let s = line.trim();
            if s.is_empty() {
                continue;
            }

            let (dir, rest) = s.split_at(1);
            let val: i32 = rest.parse().unwrap();

            if dir == "L" {
                self.moves.push(-val);
            } else {
                self.moves.push(val);
            }
        }
    }

    // ------------------------------------------------------------
    // Part 1
    // ------------------------------------------------------------
    fn part1(&mut self) -> String {
        let mut pos: i32 = 50;
        let mut count_zero = 0;

        for &delta in &self.moves {
            pos = Self::mod100(pos + delta);
            if pos == 0 {
                count_zero += 1;
            }
        }

        count_zero.to_string()
    }

    // ------------------------------------------------------------
    // Part 2
    // ------------------------------------------------------------
    fn part2(&mut self) -> String {
        let mut pos: i32 = 50;
        let mut count_zero = 0;

        for &delta in &self.moves {
            let step = if delta < 0 { -1 } else { 1 };
            let mut moved = 0;

            while moved != delta {
                pos += step;
                if pos < 0 {
                    pos += 100;
                } else if pos >= 100 {
                    pos -= 100;
                }

                if pos == 0 {
                    count_zero += 1;
                }

                moved += step;
            }
        }

        count_zero.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            "L68", "L30", "R48", "L5", "R60",
            "L55", "L1", "L99", "R14", "L82",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day01::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "3");
    }

    #[test]
    fn part2_example() {
        let mut d = Day01::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "6");
    }
}