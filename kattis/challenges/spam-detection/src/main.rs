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
	string: String // 1-100_000 characters, all whitespaces made into _ and ASCII-codes 33..=126
}
impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		Ok(Problem {
			string: input.trim().to_string()
		})
	}
}

type DecimalType = f32;
#[derive(Debug)]
struct Solution {
	// each value ordered, with precision of absolute/relative 10^-6
	whitespace_char_ratio: DecimalType,
	lowercase_char_ratio:  DecimalType,
	uppercase_char_ratio:  DecimalType,
	symbol_char_ratio:     DecimalType
}
impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		let chars: Vec<char> = problem.string.chars().collect();
		debug_assert_eq!(
			problem
				.string
				.chars()
				.filter(|c| c.is_ascii())
				.collect::<Vec<_>>(),
			chars,
			"All chars should be ASCII"
		);

		let total = chars.len() as DecimalType;

		let mut whitespace_chars = 0;
		let mut lowercase_chars = 0;
		let mut uppercase_chars = 0;
		let mut symbol_chars = 0;
		for new_char in chars.into_iter() {
			match new_char {
				c if c == '_' => whitespace_chars += 1,
				c if c.is_lowercase() => lowercase_chars += 1,
				c if c.is_uppercase() => uppercase_chars += 1,
				_ => symbol_chars += 1
			};
		}

		Self {
			whitespace_char_ratio: whitespace_chars as DecimalType / total,
			lowercase_char_ratio:  lowercase_chars as DecimalType / total,
			uppercase_char_ratio:  uppercase_chars as DecimalType / total,
			symbol_char_ratio:     symbol_chars as DecimalType / total
		}
	}
}
impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		format!(
			"{}\n{}\n{}\n{}",
			self.whitespace_char_ratio,
			self.lowercase_char_ratio,
			self.uppercase_char_ratio,
			self.symbol_char_ratio
		)
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{DecimalType, Problem, Solution};

	impl FromStr for Solution {
		type Err = String;

		//
		fn from_str(sample_output: &str) -> Result<Self, Self::Err> {
			let mut lines = sample_output.trim().lines();
			
			Ok(Self {
				whitespace_char_ratio: lines
					.next()
					.ok_or_else(|| "No next line?".to_string())?
					.parse::<DecimalType>()
					.or_else(|e| Err(e.to_string()))?,
				lowercase_char_ratio:  lines
					.next()
					.ok_or_else(|| "No next line?".to_string())?
					.parse::<DecimalType>()
					.or_else(|e| Err(e.to_string()))?,
				uppercase_char_ratio:  lines
					.next()
					.ok_or_else(|| "No next line?".to_string())?
					.parse::<DecimalType>()
					.or_else(|e| Err(e.to_string()))?,
				symbol_char_ratio:     lines
					.next()
					.ok_or_else(|| "No next line?".to_string())?
					.parse::<DecimalType>()
					.or_else(|e| Err(e.to_string()))?
			})
		}
	}
	impl PartialEq for Solution {
		fn eq(&self, other: &Self) -> bool {
			[
				(self.whitespace_char_ratio, other.whitespace_char_ratio),
				(self.lowercase_char_ratio, other.lowercase_char_ratio),
				(self.uppercase_char_ratio, other.uppercase_char_ratio),
				(self.symbol_char_ratio, other.symbol_char_ratio)
			]
			.iter()
			.all(|(self_val, other_val)| (self_val - other_val).abs() < 0.0001)
		}
	}

	seq_macro::seq!(N in 1..=2 {
		const INPUT_~N: &str = include_str!(stringify!(in_~N.txt));
		const OUTPUT_~N: &str = include_str!(stringify!(out_~N.txt));

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
			assert_eq!(solution.to_string().lines().collect::<String>(), OUTPUT_~N.lines().collect::<String>());
		}
	});
}