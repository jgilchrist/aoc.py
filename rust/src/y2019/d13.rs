use crate::{AocSolution, aoc::Solution};

pub struct Day13;

impl AocSolution for Day13 {
    type Input = String;

    fn get_input() -> &'static str {
        include_str!("d13.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::UnsolvedWithKnownAnswerFromPython(315);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::UnsolvedWithKnownAnswerFromPython(16171);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
