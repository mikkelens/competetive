#![allow(clippy::many_single_char_names)]

use std::{
	convert::Into,
	fmt::Debug,
	io::{stdin, Read},
	ops::Add,
	str::FromStr
};

fn read_stdin() -> String {
	let mut input = String::new();
	stdin()
		.read_to_string(&mut input)
		.expect("Could not read from stdin()?");
	input
}

fn main() {
	let input = read_stdin();
	eprintln!("--- INPUT:");
	eprintln!("{}", input);
	let problem: Problem = input.parse().unwrap();
	let solution: Solution = problem.into();
	eprintln!("--- OUTPUT:");
	println!("{}", solution.to_string());
}

#[derive(PartialEq, Debug, Clone)]
enum BoardData {
	Empty,
	Blocked,
	Visited
}
#[derive(Debug, Clone, PartialEq)]
struct Position {
	x: isize, // r????
	y: isize  // c????
}
impl Add<Move> for Position {
	type Output = Position;

	fn add(self, rhs: Move) -> Self::Output {
		Position {
			x: self.x + rhs.x as isize,
			y: self.y + rhs.y as isize
		}
	}
}

#[derive(Debug)]
struct ChessBoard {
	size: usize,
	data: Vec<BoardData> // size * size
}
impl ChessBoard {
	fn get_board_data(&self, pos: &Position) -> Option<&BoardData> {
		if !self.inside_bounds(pos) {
			None
		} else {
			self.data.get(self.get_index(pos))
		}
	}

	fn inside_bounds(&self, pos: &Position) -> bool {
		(pos.x >= 0 && pos.x < self.size as isize) // inside x
			&& (pos.y >= 0 && pos.y < self.size as isize) // inside out
	}

	fn get_board_data_mut(&mut self, pos: &Position) -> Option<&mut BoardData> {
		if !self.inside_bounds(pos) {
			None
		} else {
			let index = self.get_index(pos); // binding
			self.data.get_mut(index)
		}
	}

	fn get_index(&self, pos: &Position) -> usize {
		(pos.y as usize * self.size) + pos.x as usize
	}

	fn can_move(&self, pos: &Position) -> bool {
		if let Some(board_data) = self.get_board_data(pos) {
			match board_data {
				BoardData::Empty => true,
				BoardData::Blocked => false,
				BoardData::Visited => false
			}
		} else {
			false
		}
	}

	fn try_occupy(&mut self, pos: &Position) -> bool {
		if self.can_move(pos) {
			assert_eq!(*self.get_board_data(pos).unwrap(), BoardData::Empty);
			*self.get_board_data_mut(pos).unwrap() = BoardData::Visited;
			assert_eq!(*self.get_board_data(pos).unwrap(), BoardData::Visited);
			true
		} else {
			false
		}
	}
}

#[derive(Copy, Clone)]
struct Move {
	x: i8,
	y: i8
}
const KNIGHT_POSSIBLE_MOVES: [Move; 8] = [
	Move { x: 2, y: 1 },
	Move { x: 2, y: -1 },
	Move { x: -2, y: 1 },
	Move { x: -2, y: -1 },
	Move { x: 1, y: 2 },
	Move { x: 1, y: -2 },
	Move { x: -1, y: 2 },
	Move { x: -1, y: -2 }
];

#[derive(Debug)]
struct Problem {
	chess_board:        ChessBoard,
	initial_knight_pos: Position
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.trim().lines();
		let n = lines
			.next()
			.ok_or_else(|| "Could not read first line".to_string())?
			.parse::<usize>()
			.map_err(|e| e.to_string())?;

		let mut chess_board = ChessBoard {
			size: n,
			data: vec![BoardData::Empty; n * n]
		};
		let mut initial_knight_pos = None;
		for (y, line) in lines.enumerate() {
			let characters: Vec<_> = line.chars().collect();
			assert_eq!(characters.len(), n);
			for (x, character) in characters.into_iter().enumerate() {
				let pos = Position {
					x: x as isize,
					y: y as isize
				};
				if character == '#' {
					*chess_board
						.get_board_data_mut(&pos)
						.ok_or_else(|| "Board position was invalid?")? = BoardData::Blocked;
				} else if character == 'K' {
					*chess_board
						.get_board_data_mut(&pos)
						.ok_or_else(|| "Board position was invalid?")? = BoardData::Visited;
					initial_knight_pos = Some(pos);
				}
			}
		}

		Ok(Problem {
			chess_board,
			initial_knight_pos: initial_knight_pos.ok_or_else(|| "".to_string())?
		})
	}
}

const TARGET_POS: Position = Position { x: 0, y: 0 };

#[derive(Debug, PartialEq)]
enum Solution {
	Steps(usize), // some number
	Unsolvable    // -1
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		let mut board = problem.chess_board;
		let mut queue = vec![problem.initial_knight_pos];
		let mut iteration = 1;
		let found_answer = 'bfs: loop {
			let mut new_queue = Vec::new();
			if queue.is_empty() {
				break 'bfs false;
			}

			for knight_position in queue.into_iter() {
				let mut legal_new_positions = KNIGHT_POSSIBLE_MOVES
					.into_iter()
					.filter_map(|m| {
						let possible_pos = knight_position.clone() + m.clone();
						if board.try_occupy(&possible_pos) {
							Some(possible_pos)
						} else {
							None
						}
					})
					.collect::<Vec<_>>();

				for legal_new_pos in legal_new_positions.iter() {
					if *legal_new_pos == TARGET_POS {
						break 'bfs true; // found answer!
					}
				}
				new_queue.append(&mut legal_new_positions);
			}
			iteration += 1;
			queue = new_queue;
		};

		if found_answer {
			Solution::Steps(iteration)
		} else {
			Solution::Unsolvable
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		match self {
			Solution::Steps(s) => s.to_string(),
			Solution::Unsolvable => "-1".to_string()
		}
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{Problem, Solution};

	impl FromStr for Solution {
		type Err = String;

		fn from_str(output: &str) -> Result<Self, Self::Err> {
			Ok(match output.trim() {
				s if s.contains("-1") => Solution::Unsolvable,
				s => Solution::Steps(s.parse::<usize>().map_err(|e| e.to_string())?)
			})
		}
	}

	seq_macro::seq!(N in 1..=2 {
		const INPUT_~N: &str = include_str!(concat!(N, ".in"));
		const OUTPUT_~N: &str = include_str!(concat!(N, ".ans"));

		#[test]
		fn problem_parsing_~N() {
			let _problem: Problem = INPUT_~N.parse().expect("Cannot parse problem!");
		}

		#[test]
		fn solution_parsing_~N() {
			let _solution: Solution = OUTPUT_~N.parse().expect("Cannot parse problem!");
		}

		#[test]
		fn solution_matches_expectation_~N() {
			let solution: Solution = INPUT_~N
				 .parse::<Problem>()
				 .expect("Cannot parse problem!")
				 .into();
			let expectation: Solution = OUTPUT_~N
				 .parse()
				 .expect("Cannot parse expected solution!");
			assert_eq!(solution, expectation);
		}

		#[test]
		fn output_matches_sample_~N() {
			let solution: Solution = INPUT_~N
				 .parse::<Problem>()
				 .expect("Cannot parse problem!")
				 .into();
			assert_eq!(solution.to_string(), OUTPUT_~N.trim());
		}
	});
}