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
	collections::HashMap,
	path::{
		//Path,
		PathBuf
	}
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref CMD_CD: Regex = Regex::new(r"^cd (.*)$").unwrap();
	static ref OUT_LS: Regex = Regex::new(r"^(\d+) ([\w.]+)$").unwrap();
}

/// Solve Advent of Code day 07 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 07. That is to say, it is the sum of all of the directory sizes for
/// which the size is inferior to 100000 bytes. A directory and its subdirectory
/// can be counted twice.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let dtree = parse_input(data);
	dtree.values()
		.filter(|&v| *v <= 100_000)
		.sum()
}

/// Parse the day's input into a map of known directories and sizes
///
/// ### Arguments
/// 
///  - `data`: the input
///
/// ### Return Value
///
/// The built dictionary of directories, subdirectories, and all, up to root,
/// with sizes as the value
fn parse_input(data: &str) -> HashMap<String, usize> {
	let mut dtree: HashMap<String, usize> = HashMap::new();
	let mut cwd = PathBuf::from("/");
	for cmdout in data.trim().split("$ ") {
		//println!("CMDOUT: {}", cmdout);
		let first_line = cmdout.split('\n').next().unwrap();
		if let Some(v) = CMD_CD.captures(first_line) {
			//println!("{:?}", v);
			let dir = &v[1];
			if dir.starts_with('/') {
				// Absolute dir
				//println!("Going to absolute {}", dir);
				cwd = PathBuf::from(dir);
			} else if dir == ".." {
				//println!("Going up");
				cwd.pop();
			} else if dir == "." {
				//println!("Staying");
			} else {
				//println!("Going to {}", dir);
				cwd.push(dir);
			}
		} else if first_line == "ls" {
			// The first line has nothing useful
			for output_line in cmdout.split('\n').skip(1) {
				if let Some(c) = OUT_LS.captures(output_line) {
					let size: usize = c[1].parse().unwrap();
					let mut pclone = cwd.clone();
					loop {
						dtree.entry(pclone.to_str().unwrap().into())
							.and_modify(|v| *v += size)
							.or_insert(size);
						if !pclone.pop() { break; }
					}
				}
			}
		}
	}
	dtree
}

/// Solve Advent of Code day 07 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 07.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	let dtree = parse_input(data);
	let root_size = dtree.get("/").unwrap();
	*dtree.values()
		.filter(|&v| root_size - *v < 40_000_000)
		.min().unwrap()
}

// vim: set tw=80:
