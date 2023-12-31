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
	length_of_song: u32 // ticks
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		Ok(Problem {
			length_of_song: input.trim().parse::<u32>().map_err(|e| e.to_string())?
		})
	}
}

#[derive(Debug, PartialEq)]
struct Solution {
	number_of_revolutions: f32 // ticks / 4
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		Solution {
			number_of_revolutions: problem.length_of_song as f32 / 4.0
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		format!("{:?}", self.number_of_revolutions)
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
				number_of_revolutions: output.trim().parse::<f32>().map_err(|e| e.to_string())?
			})
		}
	}

	seq_macro::seq!(N in 0..=1 {
		const INPUT_~N: &str = include_str!(concat!("metronome-000", N, ".in"));
		const OUTPUT_~N: &str = include_str!(concat!("metronome-000", N, ".ans"));

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