//! This crates contains the code necessary to solve Advent of Code day 06,
//! all written in Rust.

extern crate common;
use common::read_data;
mod lib;
use lib::{solve_part_one, solve_part_two};

#[doc(hidden)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let data = read_data("input")?;
	println!("{}", solve_part_one(&data));
	println!("{}", solve_part_two(&data));
	Ok(())
}


#[cfg(test)]
mod test {
	use super::*;
	use common::test;

	test!(day06_01_example1, 1, 7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb");
	test!(day06_01_example2, 1, 5, "bvwbjplbgvbhsrlpgdmjqwftvncz");
	test!(day06_01_example3, 1, 6, "nppdvjthqldpwncqszvftbrmjlhg");
	test!(day06_01_example4, 1, 10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
	test!(day06_01_example5, 1, 11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
	test!(day06_02_example1, 2, 19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb");
	test!(day06_02_example2, 2, 23, "bvwbjplbgvbhsrlpgdmjqwftvncz");
	test!(day06_02_example3, 2, 23, "nppdvjthqldpwncqszvftbrmjlhg");
	test!(day06_02_example4, 2, 29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
	test!(day06_02_example5, 2, 26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
}
