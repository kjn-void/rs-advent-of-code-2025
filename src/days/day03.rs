use crate::days::Solution;

pub struct Day03 {
    banks: Vec<Vec<u8>>, // digits 0–9
}

impl Day03 {
    pub fn new() -> Self {
        Self { banks: Vec::new() }
    }

    fn max_joltage(&self, pick: usize) -> String {
        let mut total: i64 = 0;

        for bank in &self.banks {
            let n = bank.len();
            let mut need = pick;
            let mut stack: Vec<u8> = Vec::with_capacity(pick);

            for (i, &dig) in bank.iter().enumerate() {
                let remaining = n - i;

                while !stack.is_empty()
                    && remaining > need
                    && *stack.last().unwrap() < dig
                {
                    stack.pop();
                    need += 1;
                }

                if need > 0 {
                    stack.push(dig);
                    need -= 1;
                }
            }

            total += stack_to_number(&stack);
        }

        total.to_string()
    }
}

fn stack_to_number(stack: &[u8]) -> i64 {
    let mut val: i64 = 0;
    for &d in stack {
        val = val * 10 + d as i64;
    }
    val
}

impl Solution for Day03 {
    fn set_input(&mut self, lines: &[String]) {
        self.banks.clear();

        for line in lines {
            let digits = line
                .bytes()
                .map(|b| b - b'0')
                .collect::<Vec<_>>();
            self.banks.push(digits);
        }
    }

    fn part1(&mut self) -> String {
        self.max_joltage(2)
    }

    fn part2(&mut self) -> String {
        self.max_joltage(12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day03::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "357");
    }

    #[test]
    fn part2_example() {
        let mut d = Day03::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "3121910778619");
    }
}