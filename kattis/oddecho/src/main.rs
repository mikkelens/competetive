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
	_n:      u8,          // amount of lines
	n_lines: Vec<String>  // words, up to 100 letters long
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.lines();
		Ok(Problem {
			_n:      lines
				.next()
				.ok_or_else(|| "".to_string())?
				.parse::<u8>()
				.map_err(|e| e.to_string())
				.unwrap(),
			n_lines: lines.map(|line| line.to_string()).collect()
		})
	}
}

#[derive(Debug, PartialEq)]
struct Solution {
	odd_indexed: Vec<String>
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		Solution {
			odd_indexed: problem
				.n_lines
				.into_iter()
				.enumerate()
				.filter_map(|(index, line)| if index % 2 == 0 { Some(line) } else { None })
				.collect()
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		self.odd_indexed.join("\n")
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
				odd_indexed: output.lines().map(|line| line.to_string()).collect()
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