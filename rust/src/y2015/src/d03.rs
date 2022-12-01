use itertools::Itertools;
use std::collections::HashSet;
use utils::geometry::d2::{coordinates::CardinalDirection, vecs::Vec2};

use aoc::{AocSolution, Solution};

pub struct Day03;

impl AocSolution for Day03 {
    fn get_input() -> &'static str {
        include_str!("d03.in")
    }

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
            .collect_vec()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(2565);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let mut pos = Vec2 { x: 0, y: 0 };
        let mut visited = HashSet::from([pos]);

        for direction in input {
            pos = pos.move_in_direction(*direction);
            visited.insert(pos);
        }

        visited.len()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(2639);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let mut santa = Vec2 { x: 0, y: 0 };
        let mut robo_santa = Vec2 { x: 0, y: 0 };

        let mut visited = HashSet::from([santa, robo_santa]);

        let santas_instructions = input.iter().step_by(2).collect_vec();
        let robo_santas_instructions = input.iter().skip(1).step_by(2).collect_vec();

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