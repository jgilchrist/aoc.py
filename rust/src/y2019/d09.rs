use crate::{aoc::Solution, AocSolution};

pub struct Day09;

impl AocSolution for Day09 {
    fn get_input() -> &'static str {
        include_str!("d09.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(4261108180);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(77944);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
