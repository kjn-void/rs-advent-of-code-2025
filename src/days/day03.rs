use crate::days::Solution;

#[derive(Default)]
pub struct Day03 {
    battery_banks: Vec<Vec<u8>>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn max_joltage(&self, batteries_to_pick: usize) -> String {
        let mut total: i64 = 0;

        for bank in &self.battery_banks {
            let bank_len = bank.len();
            let mut remaining_picks = batteries_to_pick;
            let mut chosen_digits: Vec<u8> = Vec::with_capacity(batteries_to_pick);

            for (index, &digit) in bank.iter().enumerate() {
                let remaining_digits = bank_len - index;

                while !chosen_digits.is_empty()
                    && remaining_digits > remaining_picks
                    && *chosen_digits.last().unwrap() < digit
                {
                    chosen_digits.pop();
                    remaining_picks += 1;
                }

                if remaining_picks > 0 {
                    chosen_digits.push(digit);
                    remaining_picks -= 1;
                }
            }

            total += digits_to_number(&chosen_digits);
        }

        total.to_string()
    }
}

fn digits_to_number(digits: &[u8]) -> i64 {
    let mut value: i64 = 0;
    for &digit in digits {
        value = value * 10 + digit as i64;
    }
    value
}

impl Solution for Day03 {
    fn set_input(&mut self, lines: &[String]) {
        self.battery_banks.clear();

        for line in lines {
            let digits = line.bytes().map(|b| b - b'0').collect::<Vec<_>>();
            self.battery_banks.push(digits);
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
