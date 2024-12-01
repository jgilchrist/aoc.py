use aoc::prelude::*;

pub struct Day12;

impl AocSolution for Day12 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python(12053);
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = solution_from_python(320380285873116u64);
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
