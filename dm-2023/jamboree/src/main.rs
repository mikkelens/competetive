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
	_n:    usize,
	// num of items
	m:     usize,
	// scouts
	items: Vec<usize>
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		eprintln!("Parsing input:\n{}", input);
		let mut lines = input.trim().lines();
		let (n, m) = lines.next().unwrap().split_once(' ').unwrap();
		Ok(Problem {
			_n:    n.parse().unwrap(),
			m:     m.parse().unwrap(),
			items: lines
				.next()
				.unwrap()
				.trim()
				.split(' ')
				.map(|element| element.parse().unwrap())
				.collect()
		})
	}
}

#[derive(Debug, PartialEq)]
struct Solution {
	max_weight: usize // max any scout will have to carry
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		let mut sorted_items = problem.items;
		sorted_items.sort();

		let mut scouts: Vec<usize> = vec![0; problem.m];

		let mut item_stack = sorted_items.into_iter().rev();
		for scout_index in 0..problem.m {
			match item_stack.next() {
				None => break,
				Some(weight) => {
					scouts[scout_index] += weight;
				}
			}
		}
		for scout_index in (0..problem.m).rev() {
			match item_stack.next() {
				None => break,
				Some(weight) => {
					scouts[scout_index] += weight;
				}
			}
		}

		Solution {
			max_weight: scouts.into_iter().max().unwrap()
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		format!("{}", self.max_weight)
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{Problem, Solution};

	impl FromStr for Solution {
		type Err = String;

		fn from_str(output: &str) -> Result<Self, Self::Err> {
			Ok(Solution {
				max_weight: output.trim().parse().unwrap()
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
			let _solution: Solution = OUTPUT_~N.trim().parse().expect("Cannot parse problem!");
		}

		#[test]
		fn solution_matches_expectation_~N() {
			let solution: Solution = INPUT_~N
				 .parse::<Problem>()
				 .expect("Cannot parse problem!")
				 .into();
			let expectation: Solution = OUTPUT_~N
			.trim()
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