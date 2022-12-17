//! This crates contains the code necessary to solve Advent of Code day 15,
//! all written in Rust.

extern crate common;
use common::read_data;
use day15::{solve_part_one, solve_part_two};

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

	test!(day15_01_example1, 1, 26, "Sensor at x=2, y=2000008: closest beacon is at x=-2, y=2000005\nSensor at x=9, y=2000006: closest beacon is at x=10, y=2000006\nSensor at x=13, y=1999992: closest beacon is at x=15, y=1999993\nSensor at x=12, y=2000004: closest beacon is at x=10, y=2000006\nSensor at x=10, y=2000010: closest beacon is at x=10, y=2000006\nSensor at x=14, y=2000007: closest beacon is at x=10, y=2000006\nSensor at x=8, y=1999997: closest beacon is at x=2, y=2000000\nSensor at x=2, y=1999990: closest beacon is at x=2, y=2000000\nSensor at x=0, y=2000001: closest beacon is at x=2, y=2000000\nSensor at x=20, y=2000004: closest beacon is at x=25, y=2000007\nSensor at x=17, y=2000010: closest beacon is at x=21, y=2000012\nSensor at x=16, y=1999997: closest beacon is at x=15, y=1999993\nSensor at x=14, y=1999993: closest beacon is at x=15, y=1999993\nSensor at x=20, y=1999991: closest beacon is at x=15, y=1999993");
	// We can't have a test 2 because the parameters vary so wildly
	// I kept it during development and changed the code after for production
	//test!(day15_02_example1, 2, 56000011, "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3");
}
