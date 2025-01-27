use aoc::prelude::*;
use utils::geometry::d2::{coordinates::CardinalDirection, vecs::Vec2};

pub struct Day02;

struct Map {
    cells: Vec<Vec<Option<char>>>,
}

impl Map {
    fn new(cells: Vec<Vec<Option<char>>>) -> Self {
        Self { cells }
    }

    #[allow(clippy::cast_sign_loss)]
    fn at(&self, pos: Vec2) -> Option<char> {
        *self
            .cells
            .get(pos.y as usize)
            .and_then(|y| y.get(pos.x as usize))
            .unwrap_or(&None)
    }
}

fn get_instructions_from_line(line: &str) -> Vec<CardinalDirection> {
    line.chars()
        .map(|c| match c {
            'U' => CardinalDirection::North,
            'D' => CardinalDirection::South,
            'L' => CardinalDirection::West,
            'R' => CardinalDirection::East,
            _ => unreachable!(),
        })
        .collect()
}

fn run_input(
    map: &Map,
    start: impl Into<Vec2>,
    instructions: &Vec<Vec<CardinalDirection>>,
) -> String {
    let mut code = String::new();
    let mut pos = start.into();

    for line in instructions {
        for instruction in line {
            let new_pos = pos.move_in_direction(*instruction);

            let at_pos = map.at(new_pos);

            if at_pos.is_some() {
                pos = new_pos;
            }
        }

        code.push(map.at(pos).unwrap());
    }

    code
}

impl AocSolution for Day02 {
    type Input = Vec<Vec<CardinalDirection>>;
    fn process_input(input: &str) -> Self::Input {
        input.lines().map(get_instructions_from_line).collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution("78293");
    fn part1(input: &Self::Input) -> impl ToSolution {
        let map = Map::new(vec![
            vec![Some('1'), Some('2'), Some('3')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('7'), Some('8'), Some('9')],
        ]);

        run_input(&map, (1, 1), input)
    }

    const PART2_SOLUTION: SolutionStatus = solution("AC8C8");
    fn part2(input: &Self::Input) -> impl ToSolution {
        let map = Map::new(vec![
            vec![None, None, Some('1'), None, None],
            vec![None, Some('2'), Some('3'), Some('4'), None],
            vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            vec![None, Some('A'), Some('B'), Some('C'), None],
            vec![None, None, Some('D'), None, None],
        ]);

        run_input(&map, (0, 2), input)
    }
}
