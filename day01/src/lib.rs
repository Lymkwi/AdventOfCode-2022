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

// Change this on the day you start working on the puzzle
#![allow(unused_variables)]

/// Solve Advent of Code day 01 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 01.
///
/// It is going to parse the data per the problem's format, i.e., a list of
/// numbers that are one each line, with paragraphs separating the count for
/// each individual elf.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	data.trim().split("\n\n")
		.map(|elf_inv_str| elf_inv_str.split('\n')
			 .map(|cal| str::parse::<usize>(cal).unwrap())
			 .sum()
		)
		.max()
		.expect("No values found!!")
}

/// Solve Advent of Code day 01 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 01.
///
/// It is going to parse the data per the problem's format, i.e., a list of
/// numbers that are one each line, with paragraphs separating the count for
/// each individual elf.
///
/// The return value is the sum of the top 3 inventory total count
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	let mut top = data.trim().split("\n\n")
		.map(|elf_inv_str| elf_inv_str.split('\n')
			 .map(|cal| str::parse::<usize>(cal).unwrap())
			 .sum::<usize>()
		)
		.collect::<Vec<usize>>();
	top.sort_by(|a, b| b.partial_cmp(a).unwrap());
	top.iter().take(3).sum()
}

