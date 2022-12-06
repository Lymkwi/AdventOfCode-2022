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

use std::collections::HashSet;

/// Solve Advent of Code day 06 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 06.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	data.trim().chars().collect::<Vec<char>>()[..]
		.windows(4)
		.enumerate()
		.find_map(|(idx, cs)| {
			let c1 = cs[0]; let c2 = cs[1];
			let c3 = cs[2]; let c4 = cs[3];
			// Comparing 4 chars is not worth creating a vector or hashset
			if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
				Some(idx+4)
			} else {
				None
			}
		}).unwrap()
}

/// Solve Advent of Code day 06 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 06.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	data.trim().chars().collect::<Vec<char>>()[..]
		.windows(14)
		.enumerate()
		.find_map(|(idx, cs)| {
			// Comparing 14 chars is worth creating a hashset
			let chars = cs.iter().collect::<HashSet<&char>>();
			if chars.len() == 14 {
				Some(idx + 14)
			} else {
				None
			}
		}).unwrap()
}

// vim: set tw=80:
