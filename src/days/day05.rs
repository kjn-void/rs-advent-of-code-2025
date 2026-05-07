use crate::days::Solution;

#[derive(Default)]
pub struct Day05 {
    fresh_ranges: Vec<(i64, i64)>,
    available_ids: Vec<i64>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_fresh(&self, id: i64) -> bool {
        let mut lo = 0usize;
        let mut hi = self.fresh_ranges.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            let (range_start, range_end) = self.fresh_ranges[mid];
            if id < range_start {
                hi = mid;
            } else if id > range_end {
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
        self.fresh_ranges.clear();
        self.available_ids.clear();

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
                self.fresh_ranges.push((start, end));
            } else {
                let id: i64 = s.parse().unwrap();
                self.available_ids.push(id);
            }
        }

        self.fresh_ranges.sort_by_key(|range| range.0);

        let mut merged: Vec<(i64, i64)> = Vec::new();
        let mut current = self.fresh_ranges[0];

        for &(start, end) in &self.fresh_ranges[1..] {
            if start <= current.1 {
                current.1 = current.1.max(end);
            } else {
                merged.push(current);
                current = (start, end);
            }
        }
        merged.push(current);

        self.fresh_ranges = merged;
    }

    fn part1(&mut self) -> String {
        let mut count = 0;
        for &id in &self.available_ids {
            if self.is_fresh(id) {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part2(&mut self) -> String {
        let total: i64 = self
            .fresh_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
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
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
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
