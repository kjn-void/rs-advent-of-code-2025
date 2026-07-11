use crate::days::Solution;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day11 {
    graph: Vec<Vec<usize>>,
    you: Option<usize>,
    svr: Option<usize>,
    dac: Option<usize>,
    fft: Option<usize>,
    out: Option<usize>,
}

impl Day11 {
    // Counts paths from the requested node to out with a dense memo indexed by node ID.
    fn count_paths_from(&self, start: Option<usize>) -> u64 {
        fn dfs(node: usize, out: usize, graph: &[Vec<usize>], memo: &mut [Option<u64>]) -> u64 {
            if node == out {
                return 1;
            }
            if let Some(cached) = memo[node] {
                return cached;
            }
            let total = graph[node]
                .iter()
                .map(|&next| dfs(next, out, graph, memo))
                .sum();
            memo[node] = Some(total);
            total
        }

        let (Some(start), Some(out)) = (start, self.out) else {
            return 0;
        };
        dfs(start, out, &self.graph, &mut vec![None; self.graph.len()])
    }

    // Counts svr-to-out paths whose node-state mask has visited both dac and fft.
    fn count_svr_paths_via_dac_and_fft(&self) -> u64 {
        fn dfs(
            node: usize,
            mut mask: usize,
            out: usize,
            dac: usize,
            fft: usize,
            graph: &[Vec<usize>],
            memo: &mut [[Option<u64>; 4]],
        ) -> u64 {
            if node == dac {
                mask |= 1;
            } else if node == fft {
                mask |= 2;
            }
            if node == out {
                return u64::from(mask == 3);
            }
            if let Some(cached) = memo[node][mask] {
                return cached;
            }
            let total = graph[node]
                .iter()
                .map(|&next| dfs(next, mask, out, dac, fft, graph, memo))
                .sum();
            memo[node][mask] = Some(total);
            total
        }

        let (Some(svr), Some(out), Some(dac), Some(fft)) = (self.svr, self.out, self.dac, self.fft)
        else {
            return 0;
        };
        dfs(
            svr,
            0,
            out,
            dac,
            fft,
            &self.graph,
            &mut vec![[None; 4]; self.graph.len()],
        )
    }
}

impl Solution for Day11 {
    // Interns device names once and stores the wiring as a compact integer-indexed graph.
    fn set_input(&mut self, lines: &[String]) {
        let parsed: Vec<(&str, Vec<&str>)> = lines
            .iter()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                let (device, outputs) = line.split_once(':').unwrap_or((line, ""));
                Some((device.trim(), outputs.split_whitespace().collect()))
            })
            .collect();

        let mut ids = HashMap::with_capacity(parsed.len() * 2);
        for (device, outputs) in &parsed {
            if !ids.contains_key(device) {
                let id = ids.len();
                ids.insert(*device, id);
            }
            for output in outputs {
                if !ids.contains_key(output) {
                    let id = ids.len();
                    ids.insert(*output, id);
                }
            }
        }

        self.graph.clear();
        self.graph.resize_with(ids.len(), Vec::new);
        for (device, outputs) in parsed {
            let node = ids[device];
            self.graph[node].reserve(outputs.len());
            self.graph[node].extend(outputs.into_iter().map(|output| ids[output]));
        }

        self.you = ids.get("you").copied();
        self.svr = ids.get("svr").copied();
        self.dac = ids.get("dac").copied();
        self.fft = ids.get("fft").copied();
        self.out = ids.get("out").copied();
    }

    fn part1(&mut self) -> String {
        self.count_paths_from(self.you).to_string()
    }

    fn part2(&mut self) -> String {
        self.count_svr_paths_via_dac_and_fft().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn split_lines(s: &str) -> Vec<String> {
        s.lines()
            .map(|line| line.trim_end_matches('\r'))
            .filter(|line| !line.trim().is_empty())
            .map(str::to_string)
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
