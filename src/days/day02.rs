use crate::days::Solution;

#[derive(Default)]
pub struct Day02 {
    id_ranges: Vec<(i64, i64)>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

// ------------------------------------------------------------
// Helpers
// ------------------------------------------------------------

const fn pow10_table() -> [i64; 18] {
    let mut t = [0i64; 18];
    let mut x = 1i64;
    let mut i = 0usize;
    while i < t.len() {
        t[i] = x;
        x *= 10;
        i += 1;
    }
    t
}

const POW10: [i64; 18] = pow10_table();

fn smallest_repeating_block_len(digits: &str) -> usize {
    let digit_count = digits.len();
    for block_len in 1..=digit_count / 2 {
        if !digit_count.is_multiple_of(block_len) {
            continue;
        }
        let block = &digits[..block_len];
        let mut repeats = true;
        for start in (block_len..digit_count).step_by(block_len) {
            if &digits[start..start + block_len] != block {
                repeats = false;
                break;
            }
        }
        if repeats {
            return block_len;
        }
    }
    digit_count
}

// ------------------------------------------------------------
// Solution impl
// ------------------------------------------------------------

impl Solution for Day02 {
    fn set_input(&mut self, lines: &[String]) {
        self.id_ranges.clear();

        if lines.is_empty() {
            return;
        }

        let line = lines[0].trim();
        for part in line.split(',') {
            if part.is_empty() {
                continue;
            }
            let mut it = part.split('-');
            let start: i64 = it.next().unwrap().parse().unwrap();
            let end: i64 = it.next().unwrap().parse().unwrap();
            self.id_ranges.push((start, end));
        }
    }

    // --------------------------------------------------------
    // Part 1
    // --------------------------------------------------------

    fn part1(&mut self) -> String {
        let mut sum: i64 = 0;

        for &(range_start, range_end) in &self.id_ranges {
            let max_digits = range_end.to_string().len();

            for block_digits in 1..=max_digits / 2 {
                let base = POW10[block_digits];
                let repeat_factor = base + 1;

                let min_block = POW10[block_digits - 1];
                let max_block = base - 1;

                let mut candidate_min = (range_start + repeat_factor - 1) / repeat_factor;
                let mut candidate_max = range_end / repeat_factor;

                if candidate_min < min_block {
                    candidate_min = min_block;
                }
                if candidate_max > max_block {
                    candidate_max = max_block;
                }
                if candidate_min > candidate_max {
                    continue;
                }

                for block_value in candidate_min..=candidate_max {
                    sum += block_value * repeat_factor;
                }
            }
        }

        sum.to_string()
    }

    // --------------------------------------------------------
    // Part 2
    // --------------------------------------------------------

    fn part2(&mut self) -> String {
        let mut total: i64 = 0;

        for &(range_start, range_end) in &self.id_ranges {
            let max_digits = range_end.to_string().len();

            for (total_digits, &ten_len) in POW10.iter().enumerate().take(max_digits + 1).skip(2) {
                for repetitions in 2..=total_digits {
                    if total_digits % repetitions != 0 {
                        continue;
                    }

                    let block_digits = total_digits / repetitions;
                    let block_base = POW10[block_digits];
                    let repeat_factor = (ten_len - 1) / (block_base - 1);

                    let min_block = POW10[block_digits - 1];
                    let max_block = block_base - 1;

                    let mut candidate_min = (range_start + repeat_factor - 1) / repeat_factor;
                    let mut candidate_max = range_end / repeat_factor;

                    if candidate_min < min_block {
                        candidate_min = min_block;
                    }
                    if candidate_max > max_block {
                        candidate_max = max_block;
                    }
                    if candidate_min > candidate_max {
                        continue;
                    }

                    for block_value in candidate_min..=candidate_max {
                        let block_digits = block_value.to_string();

                        if smallest_repeating_block_len(&block_digits) != block_digits.len() {
                            continue;
                        }

                        total += block_value * repeat_factor;
                    }
                }
            }
        }

        total.to_string()
    }
}

// ------------------------------------------------------------
// Tests (inline, Rust-style)
// ------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec!["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124"
            .to_string()]
    }

    #[test]
    fn part1_example() {
        let mut d = Day02::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "1227775554");
    }

    #[test]
    fn part2_example() {
        let mut d = Day02::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "4174379265");
    }
}
