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
	// let problem: Input = input.parse().unwrap();
	let solution: Solution = input.parse().unwrap();
	eprintln!("--- OUTPUT:");
	println!("{}", solution.to_string());
}

// #[derive(Debug)]
// struct Input {
// 	n: usize;
// 	board: &[[&char]]
// }

// impl FromStr for Input {
// 	type Err = String;
//
// 	fn from_str(input: &str) -> Result<Self, Self::Err> {
// 		// parse input
// 		todo!()
// 		// Ok(Problem());
// 	}
// }

#[derive(Debug, PartialEq)]
struct Solution();

impl FromStr for Solution {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		todo!()c
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

	use crate::{Input, Solution};

	// impl FromStr for Solution {
	// 	type Err = String;
	//
	// 	fn from_str(output: &str) -> Result<Self, Self::Err> {
	// 		todo!()
	// 	}
	// }

	seq_macro::seq!(N in 1..=2 {
		const INPUT_~N: &str = include_str!(concat!(0, N, ".in"));
		const OUTPUT_~N: &str = include_str!(concat!(0, N, ".ans"));

		#[test]
		fn problem_parsing_~N() {
			let _problem: Input = INPUT_~N.parse().expect("Cannot parse problem!");
		}

		// #[test]
		// fn solution_parsing_~N() {
		// 	let _solution: Solution = OUTPUT_~N.parse().expect("Cannot parse problem!");
		// }

		// #[test]
		// fn solution_matches_expectation_~N() {
		// 	let solution: Solution = INPUT_~N
		// 		 .parse::<Problem>()
		// 		 .expect("Cannot parse problem!")
		// 		 .into();
		// 	let expectation: Solution = OUTPUT_~N
		// 		 .parse()
		// 		 .expect("Cannot parse expected solution!");
		// 	assert_eq!(solution, expectation);
		// }

		#[test]
		fn output_matches_sample_~N() {
			let solution: Solution = INPUT_~N
				 .parse::<Input>()
				 .expect("Cannot parse problem!")
				 .into();
			assert_eq!(solution.to_string(), OUTPUT_~N);
		}
	});
}