use crate::days::Solution;

pub struct Day02 {
    ranges: Vec<(i64, i64)>, // inclusive [L, R]
}

impl Day02 {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
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

const P10: [i64; 18] = pow10_table();

// smallest repeating block size of numeric string s
fn smallest_block(s: &str) -> usize {
    let n = s.len();
    for k in 1..=n / 2 {
        if n % k != 0 {
            continue;
        }
        let block = &s[..k];
        let mut ok = true;
        for i in (k..n).step_by(k) {
            if &s[i..i + k] != block {
                ok = false;
                break;
            }
        }
        if ok {
            return k;
        }
    }
    n
}

// ------------------------------------------------------------
// Solution impl
// ------------------------------------------------------------

impl Solution for Day02 {
    fn set_input(&mut self, lines: &[String]) {
        self.ranges.clear();

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
            self.ranges.push((start, end));
        }
    }

    // --------------------------------------------------------
    // Part 1
    // --------------------------------------------------------

    fn part1(&mut self) -> String {
        let mut sum: i64 = 0;

        for &(l, r) in &self.ranges {
            let max_digits = r.to_string().len();

            for k in 1..=max_digits / 2 {
                let base = P10[k];
                let rep_factor = base + 1;

                let d_lo = P10[k - 1];
                let d_hi = base - 1;

                let mut cand_min = (l + rep_factor - 1) / rep_factor;
                let mut cand_max = r / rep_factor;

                if cand_min < d_lo {
                    cand_min = d_lo;
                }
                if cand_max > d_hi {
                    cand_max = d_hi;
                }
                if cand_min > cand_max {
                    continue;
                }

                for dd in cand_min..=cand_max {
                    sum += dd * rep_factor;
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

        for &(l, r) in &self.ranges {
            let max_digits = r.to_string().len();

            for total_digits in 2..=max_digits {
                let ten_len = P10[total_digits];

                for m in 2..=total_digits {
                    if total_digits % m != 0 {
                        continue;
                    }

                    let k = total_digits / m;
                    let base_k = P10[k];
                    let rep_factor = (ten_len - 1) / (base_k - 1);

                    let d_lo = P10[k - 1];
                    let d_hi = base_k - 1;

                    let mut cand_min = (l + rep_factor - 1) / rep_factor;
                    let mut cand_max = r / rep_factor;

                    if cand_min < d_lo {
                        cand_min = d_lo;
                    }
                    if cand_max > d_hi {
                        cand_max = d_hi;
                    }
                    if cand_min > cand_max {
                        continue;
                    }

                    for dd in cand_min..=cand_max {
                        let ds = dd.to_string();

                        // uniqueness: dd must not have internal repetition
                        if smallest_block(&ds) != ds.len() {
                            continue;
                        }

                        total += dd * rep_factor;
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
        vec![
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124"
                .to_string(),
        ]
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