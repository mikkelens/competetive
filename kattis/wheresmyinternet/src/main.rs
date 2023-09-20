#![allow(unused_imports)]
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

fn main() {
    // take input from kattis
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // let input = include_str!("in_1.txt");

    let output = parse_and_solve_input(input);
    match &output {
        Output::Connected => println!("{}", CONNECTED),
        Output::Missing(vec) => {
            for id in vec.iter() {
                println!("{}", id);
            }
        }
    }
}

/// TODO:
/// Account for the possibility that there are more connections to COMPLETE,
/// than connections DONE (M).
/// The possible current oversight may cause more house connections to count as real (M),
/// as opposed to planned (?)

fn parse_and_solve_input(input: impl AsRef<str>) -> Output {
    // handle early end
    let sequence = {
        let mut lines = input.as_ref().lines();

        // figure out how far to look
        let first_line = lines.next().unwrap().split_once(' ').unwrap();
        let (house_count, line_count) = (
            first_line.0.parse::<usize>().unwrap(),
            first_line.1.parse::<usize>().unwrap(),
        );
        let mut lines = lines.enumerate();

        // build that far
        let mut builder = Vec::new();
        for (line_index, line) in lines.by_ref() {
            if line_index * 2 >= house_count || line_index >= line_count {
                break;
            }
            builder.push(line);
        }
        assert!(lines.next().is_none());
        builder
    };
    solve_sequence(sequence.iter())
}

const CONNECTED: &str = "Connected";

#[derive(Debug, PartialEq)]
enum Output {
    Connected,
    Missing(Vec<ID>),
}

type ID = usize;
type Connections = BTreeSet<ID>;

fn solve_sequence(lines: impl Iterator<Item = impl AsRef<str>>) -> Output {
    let mut internet_houses: Connections = Connections::from([1]);
    let mut all_houses: BTreeMap<ID, Connections> = BTreeMap::from([(1, Connections::default())]);

    // make connection tree
    for line in lines {
        let (first, second) = line.as_ref().split_once(' ').expect("could not split?");
        let (first, second) = (
            first.parse().expect("Could not parse as usize?"),
            second.parse().expect("Could not parse as usize?"),
        );
        all_houses.entry(first).or_default().insert(second);
        all_houses.entry(second).or_default().insert(first);
    }

    // connect all if possible
    let mut made_connection = true;
    while made_connection {
        made_connection = false;
        for (this, connections) in all_houses.clone() {
            if connections.contains(&1) && internet_houses.insert(this) {
                made_connection = true;
                eprintln!("Self house [{}] connected to the internet!", this);
            }
            if internet_houses.contains(&this) {
                // connect all others to internet
                for other in connections {
                    if internet_houses.insert(other) {
                        made_connection = true;
                        eprintln!("Other house [{}] connected to the internet!", other);
                    }
                }
            }
        }
    }

    let mut unconnected_houses = all_houses
        .into_keys()
        .filter(|id| !internet_houses.contains(id))
        .collect::<Vec<_>>();

    if unconnected_houses.is_empty() {
        Output::Connected
    } else {
        unconnected_houses.sort();
        Output::Missing(unconnected_houses)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_and_solve_input, solve_sequence, Output, CONNECTED};
    use std::str::FromStr;
    use test_case::test_case;

    impl FromStr for Output {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim() {
                s if s.is_empty() => Err("Nope!".to_string()),
                s if s.contains(CONNECTED) => Ok(Output::Connected),
                s => Ok(Output::Missing(
                    s.lines()
                        .map(|line| line.trim().parse::<usize>())
                        .collect::<Result<_, _>>()
                        .map_err(|e| e.to_string())?,
                )),
            }
        }
    }

    #[test_case(include_str!("out_1.txt"); "sample_1")]
    #[test_case(include_str!("out_2.txt"); "sample_2")]
    #[test_case(include_str!("out_3.txt"); "sample_3")]
    fn parse_test(sample_output: &str) {
        sample_output.parse::<Output>().unwrap();
    }

    #[test_case(include_str!("in_1.txt"), include_str!("out_1.txt"); "sample_1")]
    #[test_case(include_str!("in_2.txt"), include_str!("out_2.txt"); "sample_2")]
    #[test_case(include_str!("in_3.txt"), include_str!("out_3.txt"); "sample_3")]
    fn solve_test(sample_input: &str, sample_output: &str) {
        let solved_output = parse_and_solve_input(sample_input);
        let expected_output = sample_output.parse().unwrap();
        assert_eq!(
            solved_output, expected_output,
            "Expected {:?}!",
            expected_output
        );
    }
}