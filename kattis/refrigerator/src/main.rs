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
	car_a:         Car,
	car_b:         Car,
	refrigerators: usize // 1..=1000
}

#[derive(Debug, PartialEq)]
struct Car {
	cost_per_trip: usize, // 500..=2000
	capacity:      usize  // 10..=20
}
impl Car {
	fn min_cost_per_refrigerator(&self) -> f32 {
		self.cost_per_trip as f32 / self.capacity as f32
	}
}

impl FromStr for Problem {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// parse input
		let mut line = input
			.trim()
			.splitn(5, ' ')
			.map(|integer| integer.parse::<usize>().map_err(|e| e.to_string()));

		Ok(Problem {
			car_a:         Car {
				cost_per_trip: line
					.next()
					.ok_or_else(|| "1st element in input line missing?".to_string())??,
				capacity:      line
					.next()
					.ok_or_else(|| "2nd element in input line missing?".to_string())??
			},
			car_b:         Car {
				cost_per_trip: line
					.next()
					.ok_or_else(|| "3rd element in input line missing?".to_string())??,
				capacity:      line
					.next()
					.ok_or_else(|| "4th element in input line missing?".to_string())??
			},
			refrigerators: line
				.next()
				.ok_or_else(|| "5th element in input line missing?".to_string())??
		})
	}
}

#[derive(Debug, PartialEq)]
struct Solution {
	car_a_trips: usize,
	car_b_trips: usize,
	total_cost:  usize // 'in swedish kroner'
}
impl From<Problem> for Solution {
	fn from(problem: Problem) -> Self {
		// solve problem
		eprintln!(
			"Car A: {:?} (min cost per refrigerator: {}",
			problem.car_a,
			problem.car_a.min_cost_per_refrigerator()
		);
		eprintln!(
			"Car B: {:?} (min cost per refrigerator: {}",
			problem.car_b,
			problem.car_b.min_cost_per_refrigerator()
		);
		eprintln!("Refrigerator count: {}", problem.refrigerators);
		let (cheapest_filled, other) = if problem.car_a.min_cost_per_refrigerator()
			<= problem.car_b.min_cost_per_refrigerator()
		{
			eprintln!("Car A is cheaper for filled trips!");
			(&problem.car_a, &problem.car_b)
		} else {
			eprintln!("Car B is cheaper for filled trips!");
			(&problem.car_b, &problem.car_a)
		};
		if cheapest_filled.cost_per_trip <= other.cost_per_trip {
			// one is cheaper in all ways
			let cheapest_full_trips = problem.refrigerators / cheapest_filled.capacity + 1;
			let (car_a_trips, car_b_trips) = if cheapest_filled == &problem.car_a {
				(cheapest_full_trips, 0)
			} else {
				(0, cheapest_full_trips)
			};
			Solution {
				car_a_trips,
				car_b_trips,
				total_cost: cheapest_filled.cost_per_trip * cheapest_full_trips
			}
		} else {
			// cheapest_filled is only cheap because it is larger
			eprintln!("...but the other car is cheaper per trip.");
			let larger_cheaper = cheapest_filled;
			let smaller_expensive = other;
			let max_large_cars_needed = {
				let lower_bound = problem.refrigerators / larger_cheaper.capacity;
				if problem.refrigerators % larger_cheaper.capacity > 0 {
					lower_bound + 1
				} else {
					lower_bound
				}
			};
			(0..=max_large_cars_needed)
				.map(|large_car_trips| {
					let small_car_trips = {
						let large_cars_capacity = large_car_trips * larger_cheaper.capacity;
						if problem.refrigerators <= large_cars_capacity {
							0
						} else {
							let leftovers_in_combination =
								problem.refrigerators - large_cars_capacity;
							let lower_bound = leftovers_in_combination / smaller_expensive.capacity;
							if leftovers_in_combination % smaller_expensive.capacity > 0 {
								lower_bound + 1
							} else {
								lower_bound
							}
						}
					};
					let (car_a_trips, car_b_trips) = if larger_cheaper == &problem.car_a {
						(large_car_trips, small_car_trips)
					} else {
						(small_car_trips, large_car_trips)
					};
					Solution {
						car_a_trips,
						car_b_trips,
						total_cost: larger_cheaper.cost_per_trip * large_car_trips
							+ smaller_expensive.cost_per_trip * small_car_trips
					}
				})
				.min_by_key(|solution| solution.total_cost)
				.unwrap()
		}
	}
}

impl ToString for Solution {
	fn to_string(&self) -> String {
		// convert data to output format
		format!(
			"{} {} {}",
			self.car_a_trips, self.car_b_trips, self.total_cost
		)
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{Problem, Solution};

	impl FromStr for Solution {
		type Err = String;

		fn from_str(output: &str) -> Result<Self, Self::Err> {
			let mut line = output
				.trim()
				.splitn(3, ' ')
				.map(|integer| integer.parse::<usize>().map_err(|e| e.to_string()));
			Ok(Solution {
				car_a_trips: line
					.next()
					.ok_or_else(|| "1th element in output line missing?".to_string())??,
				car_b_trips: line
					.next()
					.ok_or_else(|| "2th element in output line missing?".to_string())??,
				total_cost:  line
					.next()
					.ok_or_else(|| "3th element in output line missing?".to_string())??
			})
		}
	}

	seq_macro::seq!(N in 1..=1 {
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