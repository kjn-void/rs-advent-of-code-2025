pub trait Solution: Send {
    fn set_input(&mut self, lines: &[String]);
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
}

pub mod day01; 
pub mod day02; 
pub mod day03; 
pub mod day04; 
pub mod day05; 
pub mod day06; 
pub mod day07; 
pub mod day08; 
pub mod day09; 
pub mod day10; 
pub mod day11;
pub mod day12;

pub fn make_solver(day: u32) -> Option<Box<dyn Solution>> {
    match day {
        01 => Some(Box::new(day01::Day01::new())),
        02 => Some(Box::new(day02::Day02::new())),
        03 => Some(Box::new(day03::Day03::new())),
        04 => Some(Box::new(day04::Day04::new())),
        05 => Some(Box::new(day05::Day05::new())),
        06 => Some(Box::new(day06::Day06::new())),
        07 => Some(Box::new(day07::Day07::new())),
        08 => Some(Box::new(day08::Day08::new())),
        09 => Some(Box::new(day09::Day09::new())),
        10 => Some(Box::new(day10::Day10::new())),
        11 => Some(Box::new(day11::Day11::default())),
        12 => Some(Box::new(day12::Day12::new())),
        _ => None,
    }
}

pub fn implemented_days() -> Vec<u32> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
}