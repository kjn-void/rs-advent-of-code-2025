use crate::days::Solution;

pub struct Day05 {
    ranges: Vec<(i64, i64)>,
    ids: Vec<i64>,
}

impl Day05 {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
            ids: Vec::new(),
        }
    }

    fn is_fresh(&self, id: i64) -> bool {
        let mut lo = 0usize;
        let mut hi = self.ranges.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            let (a, b) = self.ranges[mid];
            if id < a {
                hi = mid;
            } else if id > b {
                lo = mid + 1;
            } else {
                return true;
            }
        }
        false
    }
}

impl Solution for Day05 {
    fn set_input(&mut self, lines: &[String]) {
        self.ranges.clear();
        self.ids.clear();

        let mut section = 0;

        for line in lines {
            let s = line.trim();
            if s.is_empty() {
                section += 1;
                continue;
            }

            if section == 0 {
                let mut parts = s.split('-');
                let start: i64 = parts.next().unwrap().parse().unwrap();
                let end: i64 = parts.next().unwrap().parse().unwrap();
                self.ranges.push((start, end));
            } else {
                let id: i64 = s.parse().unwrap();
                self.ids.push(id);
            }
        }

        // Sort and merge ranges
        self.ranges.sort_by_key(|r| r.0);

        let mut merged: Vec<(i64, i64)> = Vec::new();
        let mut cur = self.ranges[0];

        for &(s, e) in &self.ranges[1..] {
            if s <= cur.1 {
                cur.1 = cur.1.max(e);
            } else {
                merged.push(cur);
                cur = (s, e);
            }
        }
        merged.push(cur);

        self.ranges = merged;
    }

    fn part1(&mut self) -> String {
        let mut count = 0;
        for &id in &self.ids {
            if self.is_fresh(id) {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part2(&mut self) -> String {
        let total: i64 = self
            .ranges
            .iter()
            .map(|(a, b)| b - a + 1)
            .sum();
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            "3-5",
            "10-14",
            "16-20",
            "12-18",
            "",
            "1",
            "5",
            "8",
            "11",
            "17",
            "32",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day05::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "3");
    }

    #[test]
    fn part2_example() {
        let mut d = Day05::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "14");
    }
}