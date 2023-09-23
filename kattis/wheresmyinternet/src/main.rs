use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Formatter},
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

    const N_LOWER_BOUND: usize = 2;
    const M_LOWER_BOUND: usize = 1;
    const M_UPPER_BOUND: usize = 200_000;

    let (expected_house_count, completed_connection_count) = {
        let (first, second) = lines.next().unwrap().split_once(' ').unwrap();
        (
            max(first.parse::<usize>().unwrap(), N_LOWER_BOUND), // N
            min(second.parse::<usize>().unwrap(), M_UPPER_BOUND), // M
        )
    };

    // collect house & connection data
    let mut house_map: BTreeMap<ID, HouseData> = BTreeMap::from([(
        1,
        HouseData {
            internet_connection: InternetState::ConfirmedConnected,
            connections: BTreeSet::new(),
        },
    )]);
    let mut house_index = N_LOWER_BOUND;
    let mut connection_index = M_LOWER_BOUND;
    for line in lines.by_ref() {
        let ids = line
            .split(' ')
            .map(|house_str| house_str.parse::<ID>().expect("Could not parse int!"));

        // count houses
        for id in ids.clone() {
            if (N_LOWER_BOUND..=expected_house_count).contains(&house_index)
                && !house_map.contains_key(&id)
            {
                assert_eq!(house_map.insert(id, HouseData::default()), None);
                eprintln!("Added house [{}] to set!", id);
                house_index += 1;
            }
        }

        // count connections
        if (M_LOWER_BOUND..=completed_connection_count).contains(&connection_index) {
            assert_eq!(2, ids.clone().count()); // "Connection must be formed between two houses!"
            let connection_pair = ids.clone().zip(ids.clone().rev());
            assert_eq!(2, connection_pair.clone().count()); //"Zip must have two iterations!"
                                                            // connect b to a, and a to b
            let connection_established = connection_pair
                .into_iter()
                .map(|(source, target)| {
                    // connect if not already connected
                    let made_connection = house_map
                        .get_mut(&source)
                        .expect("Map should already have this ID.")
                        .connections
                        .insert(target);
                    if made_connection {
                        eprintln!("Connected {source} to {target}.");
                    }
                    made_connection
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
    // assert_eq!(expected_house_count, house_map.len());

    let mut unconnected_house_ids = house_map
        .clone()
        .into_keys()
        .filter(|id| {
            eprintln!();
            !has_internet_connection(id, &mut house_map, BTreeSet::new())
        })
        // .cloned()
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
    house_map: &mut BTreeMap<ID, HouseData>,
    mut seen_ids: BTreeSet<ID>,
) -> bool {
    let info = house_map
        .get(this_id)
        .expect("Info map should always contain this");
    eprint!("[{}]", this_id);
    match info.internet_connection {
        InternetState::Unknown => {
            assert!(seen_ids.insert(*this_id));
            let connections = house_map.get(this_id).unwrap().connections.clone(); // connections remain unchanged
            connections.iter().any(|other_id| {
                !seen_ids.contains(other_id)
                    && ({
                        eprint!(" -> ");
                        let found_internet =
                            has_internet_connection(other_id, house_map, seen_ids.clone());
                        if found_internet {
                            eprint!(" ...which gives internet to [{}]", this_id);
                            let info_mut = house_map.get_mut(this_id).unwrap();
                            info_mut.internet_connection = InternetState::ConfirmedConnected;
                        }
                        found_internet
                    })
            }) || ({
                // this path is always false but we want do extra work
                house_map.get_mut(this_id).unwrap().internet_connection =
                    InternetState::ConfirmedDisconnected;
                !connections.is_empty()
                    && ({
                        eprintln!(); // print space after non-empty
                        false
                    })
            })
        }
        InternetState::ConfirmedConnected => {
            eprint!(" ...already has internet!");
            true
        }
        InternetState::ConfirmedDisconnected => false,
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct HouseData {
    internet_connection: InternetState,
    connections: BTreeSet<ID>,
}
impl Debug for HouseData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.internet_connection, self.connections)
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
enum InternetState {
    #[default]
    Unknown,
    ConfirmedConnected,
    ConfirmedDisconnected,
}

pub fn wheresmyinternet(input: String) -> String {
    let mut lines = input.lines();

    let _no_houses = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .0
        .parse::<usize>()
        .unwrap();

    let rest = lines
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));

    let mut house_map: BTreeMap<ID, HouseData> = BTreeMap::from([(
        1,
        HouseData {
            internet_connection: InternetState::ConfirmedConnected,
            ..Default::default()
        },
    )]);

    fn connect(map: &mut BTreeMap<ID, HouseData>, source: ID, target: ID) {
        map.entry(source)
            .and_modify(|source_data| {
                source_data.connections.insert(target);
            })
            .or_insert({
                HouseData {
                    connections: BTreeSet::from([target]),
                    ..Default::default()
                }
            });
    }

    for (a, b) in rest {
        connect(&mut house_map, a, b);
        connect(&mut house_map, b, a);
    }

    eprintln!("Connections *before*:\n{:?}", house_map);

    // let mut flags = vec![false; no_houses];

    // fn descent(map: &BTreeMap<ID, HouseData>, id: ID, max: usize, flags: &mut Vec<bool>) {
    //     if max == 0 || flags[id - 1] {
    //         return;
    //     }
    //     flags[id - 1] = true;
    //     let binding = HouseData::default();
    //     let connections = &map.get(&id).unwrap_or(&binding).connections; // Use unwrap_or to handle None
    //     for &neighbor in connections {
    //         if !flags[neighbor - 1] {
    //             descent(map, neighbor, max - 1, flags);
    //         }
    //     }
    // }
    // descent(&map, 1, no_houses, &mut flags);

    fn try_connect_from(
        this_id: ID,
        map: &mut BTreeMap<ID, HouseData>,
        traveled_ids: &mut BTreeSet<ID>,
    ) {
        assert!(traveled_ids.insert(this_id));
        let state = map.get(&this_id).unwrap().internet_connection.clone();
        for other_id in map.get(&this_id).unwrap().connections.clone().iter() {
            if !traveled_ids.contains(other_id) {
                if state == InternetState::ConfirmedConnected {
                    map.get_mut(other_id).unwrap().internet_connection =
                        InternetState::ConfirmedConnected;
                }
                try_connect_from(*other_id, map, traveled_ids);
                if state != InternetState::ConfirmedConnected
                    && map.get(other_id).unwrap().internet_connection
                        == InternetState::ConfirmedConnected
                {
                    map.get_mut(&this_id).unwrap().internet_connection =
                        InternetState::ConfirmedConnected;
                }
            }
        }
        if state == InternetState::Unknown {
            map.get_mut(&this_id).unwrap().internet_connection =
                InternetState::ConfirmedDisconnected
        }
    }
    try_connect_from(1, &mut house_map, &mut BTreeSet::new());

    eprintln!("Connections *after*:\n{:?}", house_map);

    let unvisited = house_map
        .iter()
        .filter(|(_id, data)| data.internet_connection != InternetState::ConfirmedConnected)
        .map(|(id, _data)| id)
        .collect::<Vec<_>>();
    if unvisited.is_empty() {
        "Connected".to_string()
    } else {
        unvisited
            .iter()
            .map(|id| format!("{}\n", id))
            .collect::<String>()
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_for_input, wheresmyinternet, Output, CONNECTED};
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
        fn alt_solve_test_~N() {
            let solved_output: Output = wheresmyinternet(INPUT_STR_~N.to_owned()).parse().unwrap();
            let expected_output: Output = OUTPUT_STR_~N.parse().unwrap();
            assert_eq!(
                solved_output, expected_output,
                "Expected {:?}!",
                expected_output
            );
        }

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