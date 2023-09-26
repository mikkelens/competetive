use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    for number in solve(input) {
        println!("{}", number);
    }
}

fn solve(input: String) -> Vec<usize> {
    let mut lines = input.lines();
    let city_count = lines.next().unwrap().parse::<usize>().unwrap();
    let mut roads_needed_per_city: Vec<usize> = Vec::new();
    for _city_num in 0..city_count {
        // parse city
        let endpoint_count = lines.next().unwrap().trim().parse::<usize>().unwrap();
        let existing_road_count = lines.next().unwrap().trim().parse::<usize>().unwrap();
        let mut endpoints: Vec<Vec<usize>> = vec![vec![]; endpoint_count];
        for _road_num in 0..existing_road_count {
            // parse road
            let (endpoint_a, endpoint_b) = lines.next().unwrap().split_once(' ').unwrap();
            let (a, b) = (
                endpoint_a.parse::<usize>().unwrap(),
                endpoint_b.parse::<usize>().unwrap(),
            );
            endpoints[a].push(b);
            endpoints[b].push(a);
        }

        // solve for needed road count in city
        roads_needed_per_city.push(find_min_roads_needed(endpoints));
    }

    fn find_min_roads_needed(city: Vec<Vec<usize>>) -> usize {
        let mut unions: Vec<Vec<usize>> = vec![];
        for (my_index, connections) in city.into_iter().enumerate() {
            let union_indexes_with_me = unions
                .iter()
                .enumerate()
                .filter_map(|(union_index, union)| {
                    if union.contains(&my_index) {
                        Some(union_index)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            match union_indexes_with_me.len() {
                0 => {
                    // add a union with me and my connections
                    let mut new_union = connections;
                    new_union.push(my_index);
                    unions.push(new_union);
                }
                1 => {
                    // add my connections if missing
                    debug_assert_eq!(union_indexes_with_me.len(), 1);
                    let my_union_index = union_indexes_with_me.first().unwrap();
                    let my_union = &mut unions[*my_union_index];
                    for connection in connections {
                        if !my_union.contains(&connection) {
                            my_union.push(connection);
                        }
                    }
                }
                _multiple => {
                    // connect multiple unions into one
                    for union_index_window in union_indexes_with_me.windows(2) {
                        let left = unions.remove(union_index_window[0]);
                        let right = &mut unions[union_index_window[1]];
                        for element in left.into_iter() {
                            if !right.contains(&element) {
                                right.push(element);
                            }
                        }
                    }
                }
            }
        }

        unions.len() - 1
    }

    roads_needed_per_city
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn parse_out(input: impl AsRef<str>) -> Vec<usize> {
        input
            .as_ref()
            .lines()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect()
    }

    #[test]
    fn test_solve() {
        let result = solve(include_str!("in.txt").to_string());
        let expectation = parse_out(include_str!("out.txt"));
        assert_eq!(result, expectation);
    }
}