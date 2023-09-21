#![allow(unused_imports)]

use std::io::Read;

fn main() {
    let input = {
        let mut reader = String::new();
        std::io::stdin()
            .read_to_string(&mut reader)
            .expect("Could not read to string!");
        reader
    };
    let ticks = input.trim().parse::<usize>().unwrap();
    let seconds = ticks as f64 / 4_f64;
    println!("{}", seconds);
}

#[test_case(include_str ! ("in_1.txt"), include_str ! ("out_1.txt"); "sample_1")]
#[test_case(include_str ! ("in_2.txt"), include_str ! ("out_2.txt"); "sample_2")]
#[test_case(include_str ! ("in_3.txt"), include_str ! ("out_3.txt"); "sample_3")]
fn solve_test(sample_input: &str, sample_output: &str) {
    let solved_output = solve_for_input(sample_input.lines());
    let expected_output = sample_output.parse().unwrap();
    assert_eq!(
        solved_output, expected_output,
        "Expected {:?}!",
        expected_output
    );
}