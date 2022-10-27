use crate::{utils::inputs, AocSolution, aoc::Solution};

pub struct Day02;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

fn paper_area_required(
    &Present {
        length,
        width,
        height,
    }: &Present,
) -> u32 {
    let sides = [length * width, width * height, height * length];

    let total_area: u32 = sides.iter().map(|s| s * 2).sum();
    let extra_paper = sides.iter().min().unwrap();

    total_area + extra_paper
}

fn ribbon_length_required(
    &Present {
        length,
        width,
        height,
    }: &Present,
) -> u32 {
    let mut dimensions = [length, width, height];
    dimensions.sort();

    let (smallest, middle) = if let [smallest, middle, _] = dimensions[..] {
        (smallest, middle)
    } else {
        unreachable!()
    };

    let smallest_perimeter = (smallest + middle) * 2;
    let volume = length * width * height;

    smallest_perimeter + volume
}

impl AocSolution for Day02 {
    type Input = Vec<Present>;
    type Output = u32;

    fn get_input() -> &'static str {
        include_str!("d02.in")
    }

    fn process_input(input: &str) -> Self::Input {
        fn parse_line(line: &str) -> Present {
            let numbers = inputs::positive_numbers(line);

            let (x1, x2, x3) = if let [x1, x2, x3] = numbers[..] {
                (x1, x2, x3)
            } else {
                unreachable!()
            };

            Present {
                length: x1,
                width: x2,
                height: x3,
            }
        }

        input.lines().map(parse_line).collect::<Vec<_>>()
    }

    const PART1_SOLUTION: Solution<Self::Output> = Solution::Solution(1586300);
    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().map(paper_area_required).sum()
    }

    const PART2_SOLUTION: Solution<Self::Output> = Solution::Solution(3737498);
    fn part2(input: &Self::Input) -> Self::Output {
        input.iter().map(ribbon_length_required).sum()
    }
}
