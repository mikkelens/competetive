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
	_n: usize,      // number of classrooms, irrelevant
	x:  usize,      // number of hand sanitizer bottles
	a:  Vec<usize>  // needed hand bottles of hand sanitizer the classrooms need
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.lines();
		let (_n, x) = lines.next().unwrap().split_once(' ').unwrap();
		Ok(Problem {
			_n: _n.parse().unwrap(),
			x:  x.parse().unwrap(),
			a:  lines
				.map(|line| line.parse::<usize>().map_err(|e| e.to_string()))
				.collect::<Result<_, Self::Err>>()?
		})
	}
}

const JEBB: &str = "Jebb";
const NEIBB: &str = "Neibb";

#[derive(Debug, PartialEq)]
enum Solution {
	Jebb,
	Neibb
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		let need_sum = problem.a.iter().sum::<usize>();
		if need_sum <= problem.x {
			Solution::Jebb
		} else {
			Solution::Neibb
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		match self {
			Solution::Jebb => JEBB,
			Solution::Neibb => NEIBB
		}
		.to_string()
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{Problem, Solution, JEBB, NEIBB};

	impl FromStr for Solution {
		type Err = String;

		fn from_str(output: &str) -> Result<Self, Self::Err> {
			Ok(match output {
				s if s.contains(JEBB) => Self::Jebb,
				s if s.contains(NEIBB) => Self::Neibb,
				_ => Err(format!("Cannot read {} as a solution!", output))?
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