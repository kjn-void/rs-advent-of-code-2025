use crate::days::Solution;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day11 {
    g: HashMap<String, Vec<String>>,
}

impl Day11 {
    fn count_paths_from(&self, start: &str) -> u64 {
        fn dfs(
            u: &str,
            g: &HashMap<String, Vec<String>>,
            memo: &mut HashMap<String, u64>,
        ) -> u64 {
            if u == "out" {
                return 1;
            }
            if let Some(&cached) = memo.get(u) {
                return cached;
            }
            let sum: u64 = g
                .get(u)
                .map(|vs| vs.iter().map(|v| dfs(v, g, memo)).sum())
                .unwrap_or(0);
            memo.insert(u.to_string(), sum);
            sum
        }

        let mut memo = HashMap::new();
        dfs(start, &self.g, &mut memo)
    }

    fn count_paths_svr_with_masks(&self) -> u64 {
        // mask bit0 = seen dac, bit1 = seen fft
        fn dfs(
            u: &str,
            mask: u8,
            g: &HashMap<String, Vec<String>>,
            memo: &mut HashMap<(String, u8), u64>,
        ) -> u64 {
            let mut m = mask;
            if u == "dac" {
                m |= 1;
            } else if u == "fft" {
                m |= 2;
            }

            if u == "out" {
                return if m == 3 { 1 } else { 0 };
            }

            let key = (u.to_string(), m);
            if let Some(&cached) = memo.get(&key) {
                return cached;
            }

            let sum: u64 = g
                .get(u)
                .map(|vs| vs.iter().map(|v| dfs(v, m, g, memo)).sum())
                .unwrap_or(0);

            memo.insert(key, sum);
            sum
        }

        let mut memo = HashMap::new();
        dfs("svr", 0, &self.g, &mut memo)
    }
}

impl Solution for Day11 {
    fn set_input(&mut self, lines: &[String]) {
        self.g.clear();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(':');
            let from = parts.next().unwrap().trim().to_string();
            let rest = parts.next().unwrap_or("").trim();
            let tos: Vec<String> = if rest.is_empty() {
                vec![]
            } else {
                rest.split_whitespace().map(|s| s.to_string()).collect()
            };
            self.g.insert(from, tos);
        }
    }

    fn part1(&mut self) -> String {
        self.count_paths_from("you").to_string()
    }

    fn part2(&mut self) -> String {
        self.count_paths_svr_with_masks().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn split_lines(s: &str) -> Vec<String> {
        s.lines()
            .map(|l| l.trim_end_matches('\r'))
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.to_string())
            .collect()
    }

    #[test]
    fn part1_example() {
        let input = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;
        let mut d = Day11::default();
        d.set_input(&split_lines(input));
        assert_eq!(d.part1(), "5");
    }

    #[test]
    fn part2_example() {
        let input = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;
        let mut d = Day11::default();
        d.set_input(&split_lines(input));
        assert_eq!(d.part2(), "2");
    }
}