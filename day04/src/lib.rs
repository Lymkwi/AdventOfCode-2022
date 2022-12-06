//! Library module with all the logic

// Clippy lints!
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

/// Solve Advent of Code day 04 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds today's input.
///
/// The shape of the input is that of `X,Y-Z,A` where `X`, `Y`, `Z` and `A` are
/// positive numbers, and those two ranges represent the ranges that elves have
/// been assigned to clean up. Our goal is to count how many pairs of elves have
/// a range where one is fully contained in the other.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 04. That is to say, return the number of pairs of ranges where one is
/// contained in the other.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	data.trim().split('\n')
		.filter_map(|line| {
			let mut ranges = line.split(',');
			let mut range_one = ranges.next().unwrap().split('-');
			let range_one_start: usize = range_one.next().unwrap().parse().unwrap();
			let range_one_end: usize = range_one.next().unwrap().parse().unwrap();
			let mut range_two = ranges.next().unwrap().split('-');
			let range_two_start: usize = range_two.next().unwrap().parse().unwrap();
			let range_two_end: usize = range_two.next().unwrap().parse().unwrap();
			if (range_one_start <= range_two_start && range_two_end <= range_one_end)
				|| (range_two_start <= range_one_start && range_one_end <= range_two_end)
			{
				Some(((range_one_start,range_one_end),(range_two_start,range_two_end)))
			} else {
				None
			}
		}) // split and parse
		.count()
}

/// Solve Advent of Code day 04 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 04. This time, it's how many ranges have any
/// overlap.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	data.trim().split('\n')
		.filter_map(|line| {
			let mut ranges = line.split(',');
			let mut range_one = ranges.next().unwrap().split('-');
			let range_one_start: usize = range_one.next().unwrap().parse().unwrap();
			let range_one_end: usize = range_one.next().unwrap().parse().unwrap();
			let mut range_two = ranges.next().unwrap().split('-');
			let range_two_start: usize = range_two.next().unwrap().parse().unwrap();
			let range_two_end: usize = range_two.next().unwrap().parse().unwrap();

			// Simplify our work
			if range_one_start <= range_two_start {
				if range_one_end >= range_two_start {
					Some(())
				} else {
					None
				}
			} else {
				if range_two_end >= range_one_start {
					Some(())
				} else {
					None
				}
			}
		}) // split and parse
		.count()
}

