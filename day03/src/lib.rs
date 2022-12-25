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

use std::{
	collections::HashSet,
	str::FromStr,
};

/// Solve Advent of Code day 03 part one
///
/// # Arguments
///
///  - `data` : a `&str` that contains today's input
///
///  One elf didn't sort they rucksack, and put one type of item across both
///  pockets (represented by both halves of an input line). For each line, we
///  have to find which item is duplicated, and put the score of that item into
///  an aggregator.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 03. It's the sum of all item scores across all lines.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	// Start by splitting each line into a knapsack, i.e. a *set* of chars
	data.trim().split('\n')
		.map(|x| Rucksack::from_str(x).expect("Invalid rucksack"))
		.map(|x| *x.common_item().expect("No intersection!"))
		.map(|c| char_score(c).unwrap_or_else(|| panic!("Invalid char {c}")))
		.sum()
}

/// Give the score of each character
fn char_score(c: char) -> Option<usize> {
	if c.is_ascii_alphabetic() {
		c.try_into().map_or(None,
							|code: u32| Some(usize::try_from(
									code - if c.is_ascii_uppercase() {
										65 - 27
									} else {
										96
									}).unwrap()
								))
	} else {
		None
	}
}

/// Each rucksack will be represented by two [`HashSet`]s of char
#[derive(Debug)]
struct Rucksack(HashSet<char>, HashSet<char>);

/// Just a custom error type for parsing
#[derive(Debug)]
struct ParseRucksackError;

impl Rucksack {
	/// Find the common item in between the pockets
	fn common_item(&self) -> Option<&char> {
		self.0.intersection(&self.1).next()
	}

	/// Create an intersection with another rucksack
	fn merge(&self) -> HashSet<char> {
		self.0.union(&self.1).copied().collect()
	}
}

impl FromStr for Rucksack {
	type Err = ParseRucksackError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let line = s.trim();
		// Try and find the size of this thing
		let tot_len = line.len();
		// If the length is odd, fail
		if tot_len % 2 == 1 {
			return Err(ParseRucksackError {})
		}
		// Otherwise, parse
		Ok(Self(
			line[..tot_len/2].chars().collect::<HashSet<_>>(),
			line[tot_len/2..].chars().collect::<HashSet<_>>()
		))
	}
}

/// Solve Advent of Code day 03 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle. That data is
///  still comprised of the rucksack lines, except the calculation method
///  changes. The items found are extracted by doing the intersection of all
///  pockets of the rucksacks of groups of three elves (in order of the input).
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 03. It is the sum of the priority scores for the
/// items found.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	let sacks: Vec<_> = data.trim().split('\n')
		.map(|x| Rucksack::from_str(x).expect("Invalid rucksack line!"))
		.collect();
	
		// Now, split into groups of 3
	sacks[..]
		.windows(3)
		.step_by(3)
		.map(|s|
			match s {
				[a, b, c] => {
					let one_merge = a.merge();
					let two_merge = b.merge();
					let thr_merge = c.merge();
					let merge = &(&one_merge & &two_merge) & &thr_merge;
					let badge = merge
						.iter().next()
						.unwrap();
					char_score(*badge).unwrap()
				},
				_ => unreachable!()
			}
		)
		.sum()
}


// vim: set tw=80:
