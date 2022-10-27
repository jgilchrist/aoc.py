use crate::{aoc::Solution, AocSolution};

pub struct Day23;

impl AocSolution for Day23 {
    type Input = String;

    fn get_input() -> &'static str {
        include_str!("d23.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = &'static str;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("78569234");
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(565615814504);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
