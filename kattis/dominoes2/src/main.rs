// n, m, l: < 10000
// domino tiles are numbered 1 -> n
// first line is setup line with info about n, m, l (3 integers)
// lines (m) after the setup line contain two integers x and y.
// if x falls, y will fall aswell
// lines (l) tell us which dominos will fall on their own

#[allow(unused_imports)]
use std::{
	collections::{HashMap, HashSet},
	io,
	io::Read
};

#[allow(unused, clippy::many_single_char_names)]
fn main() {
	let input = include_str!("in.txt");
	// let mut input = String::new();
	// io::stdin().lock().read_to_string(&mut input);

	let lines: Vec<_> = input.lines().collect();

	// read structure
	let mut index = 1;
	while index < lines.len() {
		let mut fallen_dominos: HashSet<usize> = HashSet::new(); // collected from l lines
		let mut domino_lineups: HashMap<usize, Vec<usize>> = HashMap::new(); // collected from m lines

		let first_line_in_test_case = &lines[index];
		if first_line_in_test_case.is_empty() {
			eprintln!("skipped a thing!");
		} else {
			let (n, rest) = first_line_in_test_case.split_once(' ').unwrap();
			let (m, l) = rest.split_once(' ').unwrap();
			let (n, m, l): (usize, usize, usize) =
				(n.parse().unwrap(), m.parse().unwrap(), l.parse().unwrap());
			for _ in 0..m {
				index += 1;
				let (x, y) = lines[index].split_once(' ').unwrap();
				let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
				domino_lineups.entry(x).or_default().push(y);
			}
			for _ in 0..l {
				index += 1;
				let z = lines[index].parse().unwrap();
				fallen_dominos.insert(z);
			}
			eprintln!("n: {}, m: {} l: {}", n, m, l);
		}
		index += 1;

		eprintln!("--- BEFORE CAUSE-EFFECT ---");
		eprintln!("fall_set: {:?}", fallen_dominos);
		eprintln!("knockover_map: {:?}", domino_lineups);
		let mut changes = true;
		while changes {
			changes = false;
			for key in domino_lineups.clone().keys() {
				if fallen_dominos.contains(key) {
					for next_dominos in domino_lineups.remove(key).unwrap() {
						if fallen_dominos.insert(next_dominos) {
							changes = true;
						}
					}
				}
			}
		}
		eprintln!("--- AFTER CAUSE-EFFECT ---");
		eprintln!("fall_set: {:?}", fallen_dominos);
		eprintln!("knockover_map: {:?}", domino_lineups);
		println!("{}", fallen_dominos.len());
	}
}
