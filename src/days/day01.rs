use crate::days::Solution;

#[derive(Default)]
pub struct Day01 {
    // Signed deltas: Rn => +n, Ln => -n
    rotations: Vec<i32>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    // Takes any signed dial value, wraps it onto the 0..99 dial, and returns the normalized position.
    fn dial_position(value: i32) -> i32 {
        let mut position = value % 100;
        if position < 0 {
            position += 100;
        }
        position
    }
}

impl Solution for Day01 {
    // Takes raw rotation instructions, parses them as signed click counts, and stores them for both parts.
    fn set_input(&mut self, lines: &[String]) {
        self.rotations.clear();

        for line in lines {
            let instruction = line.trim();
            if instruction.is_empty() {
                continue;
            }

            let (direction, rest) = instruction.split_at(1);
            let clicks: i32 = rest.parse().unwrap();

            if direction == "L" {
                self.rotations.push(-clicks);
            } else {
                self.rotations.push(clicks);
            }
        }
    }

    // Applies each full rotation from the starting position and returns how many rotations end at zero.
    fn part1(&mut self) -> String {
        let mut position: i32 = 50;
        let mut zero_hits = 0;

        for &rotation in &self.rotations {
            position = Self::dial_position(position + rotation);
            if position == 0 {
                zero_hits += 1;
            }
        }

        zero_hits.to_string()
    }

    // Counts zero crossings arithmetically for each rotation and returns their total.
    fn part2(&mut self) -> String {
        let mut position: i32 = 50;
        let mut zero_hits = 0;

        for &rotation in &self.rotations {
            if rotation >= 0 {
                zero_hits += (position + rotation) / 100;
            } else {
                let distance = -rotation;
                let clicks_to_zero = (100 - position) % 100;
                zero_hits += (distance + clicks_to_zero) / 100;
            }

            position = Self::dial_position(position + rotation);
        }

        zero_hits.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
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

    #[test]
    fn arithmetic_crossing_count_matches_click_simulation() {
        fn simulate(rotations: &[i32]) -> i32 {
            let mut position = 50;
            let mut hits = 0;
            for &rotation in rotations {
                let step = rotation.signum();
                for _ in 0..rotation.unsigned_abs() {
                    position = Day01::dial_position(position + step);
                    hits += i32::from(position == 0);
                }
            }
            hits
        }

        for first in (-250..=250).step_by(17) {
            for second in (-250..=250).step_by(29) {
                let rotations = [first, second, -first / 2];
                let mut day = Day01 {
                    rotations: rotations.to_vec(),
                };
                assert_eq!(day.part2(), simulate(&rotations).to_string());
            }
        }
    }
}
