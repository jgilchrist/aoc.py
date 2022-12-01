use aoc::{AocSolution, Solution};

pub struct Day02;

impl AocSolution for Day02 {
    fn get_input() -> &'static str {
        include_str!("d02.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(5478);
    fn part1(_input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = &'static str;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("qyzphxoiseldjrntfygvdmanu");
    fn part2(_input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}