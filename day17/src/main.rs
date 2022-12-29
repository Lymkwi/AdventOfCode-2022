//! This crates contains the code necessary to solve Advent of Code day 17,
//! all written in Rust.

extern crate common;
use common::read_data;
use day17::{solve_part_one, solve_part_two};

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

	test!(day17_01_example1, 1, 3068, ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
	test!(day17_02_example1, 2, 1_514_285_714_288, ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
}
