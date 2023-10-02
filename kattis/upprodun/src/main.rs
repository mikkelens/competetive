#![allow(clippy::many_single_char_names)]

use std::{
	collections::HashSet,
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
	n: usize, // rooms (divisor)
	m: usize  // teams (elements)
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.lines();
		Ok(Problem {
			n: lines.next().unwrap().parse().unwrap(),
			m: lines.next().unwrap().parse().unwrap()
		})
	}
}

#[derive(Debug)]
struct Solution {
	team_placement: Vec<usize> // ordered list of how many teams are in each room
}
impl PartialEq for Solution {
	fn eq(&self, other: &Self) -> bool {
		// same length and same elements
		self.team_placement.len() == other.team_placement.len() && {
			self.team_placement
				.iter()
				.cloned()
				.collect::<HashSet<usize>>()
				== self
					.team_placement
					.iter()
					.cloned()
					.collect::<HashSet<usize>>()
		}
	}
}
impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem

		let minimum = problem.m / problem.n;
		let leftovers = problem.m % problem.n;
		debug_assert!(leftovers < problem.n);

		let mut team_placement = vec![minimum; problem.n];
		for index in 0..leftovers {
			team_placement[index] += 1;
		}

		Solution { team_placement }
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		self.team_placement
			.iter()
			.map(|teams_in_room| (0..*teams_in_room).map(|_| '*').collect::<String>())
			.map(|line| format!("{}\n", line))
			.collect::<String>()
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
				team_placement: output.lines().map(|line| line.chars().count()).collect()
			})
		}
	}

	seq_macro::seq!(N in 1..=4 {
		const INPUT_~N: &str = include_str!(concat!(0, N, ".in"));
		const OUTPUT_~N: &str = include_str!(concat!(0, N, ".ans"));

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

		// #[test]
		// fn output_matches_sample_~N() {
		// 	let solution: Solution = INPUT_~N
		// 		 .parse::<Problem>()
		// 		 .expect("Cannot parse problem!")
		// 		 .into();
		// 	assert_eq!(solution.to_string(), OUTPUT_~N);
		// }
	});
}