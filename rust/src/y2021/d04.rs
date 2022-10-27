use std::collections::HashSet;

use crate::{aoc::Solution, utils::inputs::comma_separated_integers, AocSolution};

pub struct Day04;

#[derive(Debug, Clone)]
pub struct Board {
    numbers: Vec<Vec<u32>>,
    all_numbers: HashSet<u32>,
    seen: HashSet<u32>,
    last_seen: Option<u32>,
}

impl Board {
    fn new(numbers: Vec<Vec<u32>>) -> Self {
        let mut all_numbers = HashSet::new();
        for row in numbers.iter() {
            for n in row {
                all_numbers.insert(*n);
            }
        }

        Board {
            numbers,
            all_numbers,
            seen: HashSet::new(),
            last_seen: None,
        }
    }

    fn mark_seen(&mut self, n: u32) {
        if !self.all_numbers.contains(&n) {
            return;
        }

        self.seen.insert(n);
        self.last_seen = Some(n);
    }

    fn is_winning(&self) -> bool {
        self.has_winning_rows() || self.has_winning_columns()
    }

    fn has_winning_rows(&self) -> bool {
        self.numbers
            .iter()
            .any(|row| row.iter().all(|num| self.seen.contains(num)))
    }

    fn has_winning_columns(&self) -> bool {
        (0..self.numbers.len())
            .into_iter()
            .any(|col| self.numbers.iter().all(|row| self.seen.contains(&row[col])))
    }

    fn score(&self) -> u32 {
        let unseen_numbers = self.all_numbers.difference(&self.seen);
        let last_seen = self.last_seen.unwrap();

        unseen_numbers.sum::<u32>() * last_seen
    }
}

impl AocSolution for Day04 {
    fn get_input() -> &'static str {
        include_str!("d04.in")
    }

    type Input = (Vec<u32>, Vec<Board>);
    fn process_input(input: &str) -> Self::Input {
        let mut file = input.lines();

        let bingo_numbers = comma_separated_integers(file.next().unwrap());

        let mut boards: Vec<Board> = vec![];
        let mut board_lines: Vec<Vec<u32>> = vec![];

        for line in file {
            if line.is_empty() {
                let board = Board::new(board_lines.clone());
                boards.push(board);
                board_lines = vec![];
            } else {
                let line_numbers = line
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect();
                board_lines.push(line_numbers);
            }
        }

        (bingo_numbers, boards)
    }

    type Part1Output = u32;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solution(4662);
    fn part1(i: &Self::Input) -> Self::Part1Output {
        let numbers = i.0.clone();
        let mut boards = i.1.clone();

        for n in numbers {
            for board in boards.iter_mut() {
                board.mark_seen(n);
                if board.is_winning() {
                    return board.score();
                }
            }
        }

        unreachable!();
    }

    type Part2Output = u32;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solution(12080);
    fn part2(i: &Self::Input) -> Self::Part2Output {
        let numbers = i.0.clone();
        let mut boards = i.1.clone();
        let mut winning_boards: Vec<Board> = vec![];

        for n in numbers {
            for board in boards.iter_mut() {
                board.mark_seen(n);

                if board.is_winning() {
                    winning_boards.push(board.clone());
                }
            }

            boards.retain(|b| !b.is_winning())
        }

        winning_boards.last().unwrap().score()
    }
}
