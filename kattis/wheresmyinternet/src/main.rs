use std::cmp::{max, min};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

fn main() {
    // take input from kattis
    let mut input = String::new();
    let _result = std::io::stdin()
        .read_to_string(&mut input)
        .expect("Could not read from stdin()");

    let output = solve_for_input(input);
    match &output {
        Output::Connected => println!("{}", CONNECTED),
        Output::Missing(vec) => {
            for id in vec.iter() {
                println!("{}", id);
            }
        }
    }
}

const CONNECTED: &str = "Connected";

#[derive(Debug, PartialEq)]
enum Output {
    Connected,
    Missing(Vec<ID>),
}

type ID = usize;

fn solve_for_input(input: impl AsRef<str>) -> Output {
    eprintln!(
        "Trying to solve for the following input:\n{}",
        input.as_ref()
    );
    let mut lines = input.as_ref().lines();

    const CONNECTION_LIMIT: usize = 200_000;
    let (expected_house_count, completed_connection_count) = {
        let line = lines.next().unwrap();
        let (first, second) = line.split_once(' ').unwrap();
        (
            max(first.parse::<usize>().unwrap(), 1),                 // N
            min(second.parse::<usize>().unwrap(), CONNECTION_LIMIT), // M
        )
    };

    // collect house & connection data
    let mut house_map: BTreeMap<ID, BTreeSet<ID>> = BTreeMap::from([(1, BTreeSet::new())]);
    let mut house_index = 1;
    let mut connection_index = 0;
    for line in lines.by_ref() {
        let ids = line
            .split(' ')
            .map(|house_str| house_str.parse::<ID>().expect("Could not parse int!"));

        // count houses
        for id in ids.clone() {
            if (0..=expected_house_count).contains(&house_index)
                && !house_map.contains_key(&id)
                && house_map.insert(id, BTreeSet::new()).is_none()
            {
                eprintln!("Added house [{}] to set!", id);
                house_index += 1;
            }
        }

        // count connections
        if (0..=completed_connection_count).contains(&connection_index) {
            assert_eq!(
                2,
                ids.clone().count(),
                "Connection must be formed between two houses!"
            );
            // create iter with pairs for elements
            let connection_pair = ids.clone().zip(ids.clone().rev());
            assert_eq!(
                2,
                connection_pair.clone().count(),
                "Zip must have two iterations!"
            );
            // connect a with b
            let connection_established = connection_pair
                .into_iter()
                .map(|(source, target)| {
                    // connect if not already connected
                    if house_map
                        .get_mut(&source)
                        .expect("Map should already have this ID.")
                        .insert(target)
                    {
                        eprintln!("Connected {source} to {target}.");
                        true
                    } else {
                        false
                    }
                    // evaluate "any" after doing full iter
                })
                .reduce(|a, b| a || b)
                .expect("Connection must be formed between two houses!");
            if connection_established {
                connection_index += 1;
            }
        }
    }

    eprintln!("house_map after parsing:{:?}\n", house_map);
    assert_eq!(expected_house_count, house_map.len());

    let mut known_connected_ids: BTreeSet<ID> = BTreeSet::from([1]);
    let mut already_checked_ids = BTreeSet::new();
    let mut unconnected_house_ids = house_map
        .iter()
        .filter(|(&id, _connections)| {
            eprintln!();
            !has_internet_connection(
                &id,
                &house_map,
                &mut known_connected_ids,
                &mut already_checked_ids,
            )
        })
        .map(|(id, _house)| *id)
        .collect::<Vec<_>>();

    if unconnected_house_ids.is_empty() {
        Output::Connected
    } else {
        unconnected_house_ids.sort();
        Output::Missing(unconnected_house_ids)
    }
}

fn has_internet_connection(
    this_id: &ID,
    connection_map: &BTreeMap<ID, BTreeSet<ID>>,
    known_internet_ids: &mut BTreeSet<ID>,
    already_checked_ids: &mut BTreeSet<ID>,
) -> bool {
    eprint!("[{}]", this_id);
    if known_internet_ids.contains(this_id) {
        eprint!(" ...already has internet!");
        true
    } else {
        already_checked_ids.insert(*this_id)
            && ({
                let result = connection_map.get(this_id).unwrap().iter().any(|other_id| {
                    eprint!(" -> ");
                    if has_internet_connection(
                        other_id,
                        connection_map,
                        known_internet_ids,
                        already_checked_ids,
                    ) {
                        eprint!(" ...which gives internet to [{}]", this_id);
                        assert!(known_internet_ids.insert(*this_id));
                        true
                    } else {
                        false
                    }
                });
                if !connection_map.get(this_id).unwrap().is_empty() {
                    eprintln!();
                }
                result
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_for_input, Output, CONNECTED};
    use seq_macro::seq;
    use std::str::FromStr;

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

    seq!(N in 1..=3 {
        const INPUT_STR_~N: &str = include_str!(stringify!(in_~N.txt));
        const OUTPUT_STR_~N: &str = include_str!(stringify!(out_~N.txt));

        #[test]
        fn parse_test_~N() {
            OUTPUT_STR_~N.parse::<Output>().unwrap();
        }

        #[test]
        fn solve_test_~N() {
            let solved_output = solve_for_input(INPUT_STR_~N);
            let expected_output = OUTPUT_STR_~N.parse().unwrap();
            assert_eq!(
                solved_output, expected_output,
                "Expected {:?}!",
                expected_output
            );
        }
    });
}