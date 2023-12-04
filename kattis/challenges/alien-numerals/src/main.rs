#![allow(clippy::many_single_char_names)]

use std::{
	collections::HashMap,
	fmt::Write,
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
	_case_number: usize, // 'T <= 100', presumably a number 1..=100
	cases:        Vec<Case>
}

#[derive(Debug)]
struct Case {
	alien_number:    String, // the number, can be any combination of human chars
	source_language: Vec<char>, // char to value, combine to get whole number
	target_language: Vec<char>  // value to char, combine chars to represent value
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut lines = input.lines();
		let _case_number = lines
			.next()
			.ok_or_else(|| "No first line!".to_string())?
			.parse::<usize>()
			.map_err(|e| e.to_string())?;

		Ok(Problem {
			_case_number,
			cases: lines
				.map(|line| {
					let Some((alien_number, rest)) = line.split_once(' ') else {
						return None;
					};
					let Some((source_language, target_language)) = rest.split_once(' ') else {
						return None;
					};
					let (source_language, target_language) = (
						source_language.chars().collect(),
						target_language.chars().collect()
					);
					Some(Case {
						alien_number: alien_number.to_string(),
						source_language,
						target_language
					})
				})
				.collect::<Option<Vec<_>>>()
				.ok_or_else(|| "A line could not be parsed!".to_string())?
		})
	}
}

#[derive(Debug, PartialEq)]
struct Solution {
	outputs: Vec<String>
}

impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		Solution {
			outputs: problem
				.cases
				.into_iter()
				.map(|case| {
					eprintln!(
						"\n> Convert number '{}' in [{}] to [{}]:",
						case.alien_number,
						case.source_language.iter().collect::<String>(),
						case.target_language.iter().collect::<String>()
					);
					// translate to computer number
					let source_base = case.source_language.len();
					let source_char_value_map: HashMap<char, usize> = case
						.source_language
						.into_iter()
						.enumerate()
						.map(|(index, char)| (char, index))
						.collect();
					let mut total = 0;
					for (count, source_char) in case.alien_number.chars().rev().enumerate() {
						let value = *source_char_value_map.get(&source_char).unwrap();
						eprintln!(" | Char '{}' = {}", source_char, value);
						total += value * source_base.pow(count as u32);
					}
					eprintln!(" - Number '{}' = {}", case.alien_number, total);

					// translate to new number system
					let target_base = case.target_language.len();
					let mut digits_rev: Vec<char> = Vec::new();
					let mut remaining_value = total;
					loop {
						let rest = remaining_value % target_base;
						remaining_value /= target_base;
						let new_first_digit = case.target_language[rest];
						eprintln!(" | Value {} is {}", rest, new_first_digit);
						digits_rev.push(new_first_digit);
						if remaining_value == 0 {
							break;
						}
					}
					let target_number_string = digits_rev.into_iter().rev().collect();
					eprintln!("< {:?}", target_number_string);
					target_number_string
				})
				.collect()
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		self.outputs
			.iter()
			.enumerate()
			.fold(String::new(), |mut builder, (index, output)| {
				let _ = writeln!(builder, "Case #{}: {}", index + 1, output);
				builder
			})
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{Problem, Solution};

	impl FromStr for Solution {
		type Err = String;

		fn from_str(answer: &str) -> Result<Self, Self::Err> {
			Ok(Solution {
				outputs: answer
					.trim()
					.lines()
					.map(|line| {
						if let Some((_lhs, rhs)) = line.split_once(": ") {
							Some(rhs.to_string())
						} else {
							None
						}
					})
					.collect::<Option<_>>()
					.ok_or_else(|| "Not all lines could be deciphered as intended".to_string())?
			})
		}
	}

	const INPUT: &str = include_str!("sample.in");
	const OUTPUT: &str = include_str!("sample.ans");

	#[test]
	fn problem_parsing() {
		let _problem: Problem = INPUT.parse().expect("Cannot parse problem!");
	}

	#[test]
	fn solution_parsing() {
		let _solution: Solution = OUTPUT.parse().expect("Cannot parse problem!");
	}

	#[test]
	fn solution_matches_expectation() {
		let solution: Solution = INPUT
			.parse::<Problem>()
			.expect("Cannot parse problem!")
			.into();
		let expectation: Solution = OUTPUT.parse().expect("Cannot parse expected solution!");
		assert_eq!(solution, expectation);
	}

	#[test]
	fn output_matches_sample() {
		let solution: Solution = INPUT
			.parse::<Problem>()
			.expect("Cannot parse problem!")
			.into();
		assert_eq!(solution.to_string(), OUTPUT);
	}
}