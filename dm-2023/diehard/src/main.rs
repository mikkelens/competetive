#![allow(clippy::many_single_char_names)]

use std::{
	io::{stdin, Read},
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

#[derive(Debug)]
struct Problem {
	die1: Vec<usize>,
	die2: Vec<usize>,
	die3: Vec<usize>
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.lines();
		Ok(Problem {
			die1: lines
				.next()
				.unwrap()
				.split(' ')
				.map(|num| num.parse().unwrap())
				.collect(),
			die2: lines
				.next()
				.unwrap()
				.split(' ')
				.map(|num| num.parse().unwrap())
				.collect(),
			die3: lines
				.next()
				.unwrap()
				.split(' ')
				.map(|num| num.parse().unwrap())
				.collect()
		})
	}
}

/// - IF NO CLEAR THIRD DICE BUT UNEQUAL SUMS: "No dice"
/// (one dice always loses AKA has bad odds against both dice)
/// - IF TWO BEST DICE HAVE EQUAL SIDES (all one number): "No dice"
/// - OUTPUT BEST DICE, OR FIRST IF EQUAL ODDS: "{num}"
#[derive(PartialEq)]
enum DiceSideSituation {
	AllEqual(usize),
	// compare dice with one another for final "no dice" check
	Different
}

struct DiceOutcome {
	self_win_sum: usize,
	situation:    DiceSideSituation
}
impl From<(&Vec<usize>, &Vec<usize>)> for DiceOutcome {
	fn from((a, b): (&Vec<usize>, &Vec<usize>)) -> Self {
		let mut all_equal_sides = true;
		let mut win_sum = 0;
		let first_a = a[0];
		for a_side in a.into_iter() {
			if all_equal_sides && first_a != *a_side {
				all_equal_sides = false;
			}

			for b_side in b.into_iter() {
				if all_equal_sides && first_a != *b_side {
					all_equal_sides = false;
				}

				if a_side > b_side {
					win_sum += 1
				}
			}
		}

		DiceOutcome {
			self_win_sum: win_sum,
			situation:    if all_equal_sides {
				DiceSideSituation::AllEqual(first_a)
			} else {
				DiceSideSituation::Different
			}
		}
	}
}

#[derive(Debug, PartialEq)]
enum Solution {
	Dice1,
	Dice2,
	Dice3,
	NoDice
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem

		let die1_vs_2 = DiceOutcome::from((&problem.die1, &problem.die2));
		let die2_vs_1 = DiceOutcome::from((&problem.die2, &problem.die1));
		let die3_vs_1 = DiceOutcome::from((&problem.die3, &problem.die1));

		todo!()
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		format!("{:?}", self)
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{Problem, Solution};

	impl FromStr for Solution {
		type Err = String;

		fn from_str(output: &str) -> Result<Self, Self::Err> {
			todo!()
		}
	}

	seq_macro::seq!(N in 1..=4 {
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
			assert_eq!(solution.to_string(), OUTPUT_~N);
		}
	});
}