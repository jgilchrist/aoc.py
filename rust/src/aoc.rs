pub type TestDefinition<Output> = (&'static str, Option<Output>, Option<Output>);

pub trait AocSolution {
    type Input;
    type Output: std::fmt::Display + Eq;

    const SOLVED: bool = true;

    fn tests() -> Vec<TestDefinition<Self::Output>> {
        vec![]
    }

    fn get_input() -> &'static str;
    fn process_input(input: &str) -> Self::Input;

    const PART1_SOLUTION: Option<Self::Output>;
    fn part1(i: &Self::Input) -> Self::Output;

    const PART2_SOLUTION: Option<Self::Output>;
    fn part2(i: &Self::Input) -> Self::Output;
}

pub struct Unsolved;
impl AocSolution for Unsolved {
    type Input = ();
    type Output = &'static str;

    const SOLVED: bool = false;

    fn get_input() -> &'static str {
        unimplemented!()
    }

    fn process_input(_: &str) -> Self::Input {
        unimplemented!()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(_: &Self::Input) -> Self::Output {
        unimplemented!()
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(_: &Self::Input) -> Self::Output {
        unimplemented!()
    }
}

pub trait AocYear {
    type D01: AocSolution;
    type D02: AocSolution;
    type D03: AocSolution;
    type D04: AocSolution;
    type D05: AocSolution;
    type D06: AocSolution;
    type D07: AocSolution;
    type D08: AocSolution;
    type D09: AocSolution;
    type D10: AocSolution;
    type D11: AocSolution;
    type D12: AocSolution;
    type D13: AocSolution;
    type D14: AocSolution;
    type D15: AocSolution;
    type D16: AocSolution;
    type D17: AocSolution;
    type D18: AocSolution;
    type D19: AocSolution;
    type D20: AocSolution;
    type D21: AocSolution;
    type D22: AocSolution;
    type D23: AocSolution;
    type D24: AocSolution;
    type D25: AocSolution;
}
