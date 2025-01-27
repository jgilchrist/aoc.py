use aoc::prelude::*;
use utils::prelude::*;

use utils::geometry::d2::{coordinates::CardinalDirection, vecs::Vec2};

pub struct Day03;

impl AocSolution for Day03 {
    type Input = Vec<CardinalDirection>;
    fn process_input(input: &str) -> Self::Input {
        input
            .trim()
            .chars()
            .map(|c| match c {
                '^' => CardinalDirection::North,
                'v' => CardinalDirection::South,
                '<' => CardinalDirection::West,
                '>' => CardinalDirection::East,
                _ => unreachable!(),
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(2565);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut pos = Vec2 { x: 0, y: 0 };
        let mut visited: Set<Vec2> = Set::from_array([pos]);

        for direction in input {
            pos = pos.move_in_direction(*direction);
            visited.insert(pos);
        }

        visited.len()
    }

    const PART2_SOLUTION: SolutionStatus = solution(2639);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut santa = Vec2 { x: 0, y: 0 };
        let mut robo_santa = Vec2 { x: 0, y: 0 };

        let mut visited: Set<Vec2> = Set::from_array([santa, robo_santa]);

        let santas_instructions = input.iter().step_by(2).collect::<Vec<_>>();
        let robo_santas_instructions = input.iter().skip(1).step_by(2).collect::<Vec<_>>();

        for direction in santas_instructions {
            santa = santa.move_in_direction(*direction);
            visited.insert(santa);
        }

        for direction in robo_santas_instructions {
            robo_santa = robo_santa.move_in_direction(*direction);
            visited.insert(robo_santa);
        }

        visited.len()
    }
}
