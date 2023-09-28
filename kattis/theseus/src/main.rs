#![allow(clippy::many_single_char_names)]

use std::{
	collections::HashMap,
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

// The essential question: What 1-indexed string was the last unique string?
#[derive(Debug)]
struct Problem {
	p:   usize, // number of parts the boats consists of (1 to 1000)
	_n:  usize, // number of days in the boating season (1 to 1000)
	w_i: Vec<String>  /* boat part words (each string in letters a-z + _, 1 to 20 characters;
	             * at most p distinct strings) */
}
impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.trim().lines();
		let (p, n) = lines.next().unwrap().split_once(' ').unwrap();
		Ok(Problem {
			p:   p.parse().unwrap(),
			_n:  n.parse().unwrap(),
			w_i: lines.map(|line| line.to_string()).collect()
		})
	}
}

#[derive(Debug, PartialEq, Eq)]
enum Solution {
	ParadoxAvoided, // theseus did not replace all the parts (P < unique elements of w_i)
	DayOfReplace(usize)  // the day he did replace all the parts (P >= unique elements of w_i)
}
impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		let mut unique_parts: HashMap<&String, bool> =
			problem.w_i.iter().map(|word| (word, false)).collect();
		eprintln!("UNIQUE: {:?}", unique_parts.keys().collect::<Vec<_>>());
		if unique_parts.len() < problem.p {
			eprintln!("not enough unique parts to have 'paradox'");
			Solution::ParadoxAvoided
		} else {
			for (index, word) in problem.w_i.iter().enumerate() {
				eprintln!("word {} on day {}", word, index + 1);
				*unique_parts.get_mut(word).unwrap() = true;
				if unique_parts.values().all(|replaced| *replaced) {
					eprintln!("all parts replaced!");
					return Solution::DayOfReplace(index + 1);
				}
			}
			unreachable!()
		}
	}
}

const PARADOX_STR: &str = "paradox avoided";

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		match self {
			Solution::ParadoxAvoided => PARADOX_STR.to_string(),
			Solution::DayOfReplace(day) => day.to_string()
		}
	}
}

#[cfg(test)]
mod tests {
	use std::{num::ParseIntError, str::FromStr};

	use crate::{Problem, Solution, PARADOX_STR};

	impl FromStr for Solution {
		type Err = ParseIntError;

		fn from_str(sample_output: &str) -> Result<Self, Self::Err> {
			Ok(if sample_output.contains(PARADOX_STR) {
				Solution::ParadoxAvoided
			} else {
				Solution::DayOfReplace(sample_output.trim().parse()?)
			})
		}
	}

	seq_macro::seq!(N in 1..=2 {
		const INPUT_SAMPLE_~N: &str = include_str!(stringify!(in_~N.txt));
		const OUTPUT_SAMPLE_~N: &str = include_str!(stringify!(out_~N.txt));

		#[test]
		fn problem_parsing_~N() {
			let _problem: Problem = INPUT_SAMPLE_~N.parse().expect("Cannot parse problem!");
		}

		#[test]
		fn solution_parsing_~N() {
			let _solution: Solution = OUTPUT_SAMPLE_~N.parse().expect("Cannot parse problem!");
		}

		#[test]
		fn solution_matches_sample_~N() {
			let solution: Solution = INPUT_SAMPLE_~N
				 .parse::<Problem>()
				 .expect("Cannot parse problem!")
				 .into();
			let expectation: Solution = OUTPUT_SAMPLE_~N
				 .parse()
				 .expect("Cannot parse expected solution!");
			assert_eq!(solution, expectation);
		}

		#[test]
		fn output_matches_sample_~N() {
			let solution: Solution = INPUT_SAMPLE_~N
				 .parse::<Problem>()
				 .expect("Cannot parse problem!")
				 .into();
			assert_eq!(solution.to_string(), OUTPUT_SAMPLE_~N.trim());
		}
	});
}