use aoc::{AocSolution, Solution};

pub struct Day14;

impl AocSolution for Day14 {
    fn get_input() -> &'static str {
        include_str!("d14.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(7440382076205);
    fn part1(_input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(4200656704538);
    fn part2(_input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
