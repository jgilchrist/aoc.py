use aoc::{AocSolution, Solution};

pub struct Day05;

impl AocSolution for Day05 {
    fn get_input() -> &'static str {
        include_str!("d05.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = &'static str;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("c6697b55");
    fn part1(_input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = &'static str;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("8c35d1ab");
    fn part2(_input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}