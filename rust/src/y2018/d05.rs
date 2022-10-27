use crate::{aoc::Solution, AocSolution};

pub struct Day05;

impl AocSolution for Day05 {
    type Input = String;

    fn get_input() -> &'static str {
        include_str!("d05.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(10564);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(6336);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
