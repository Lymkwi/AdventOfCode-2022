//! This crates contains the code necessary to solve Advent of Code day 09,
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

	test!(day09_01_example1, 1, 13, "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
	test!(day09_02_example1, 2, 1, "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
	test!(day09_02_example2, 2, 36, "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20");
}
