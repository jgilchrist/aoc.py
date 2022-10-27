use crate::{aoc::Solution, AocSolution};

pub struct Day25;

impl AocSolution for Day25 {
    fn get_input() -> &'static str {
        include_str!("d25.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(6421487);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::MerryChristmas;
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
