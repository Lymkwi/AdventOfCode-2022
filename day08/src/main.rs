//! This crates contains the code necessary to solve Advent of Code day 08,
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

	test!(day08_01_example1, 1, 21, "30373\n25512\n65332\n33549\n35390");
	test!(day08_02_example1, 2, 8, "30373\n25512\n65332\n33549\n35390");
}
