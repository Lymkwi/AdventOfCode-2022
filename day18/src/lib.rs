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

use std::collections::{VecDeque, HashSet};

/// Solve Advent of Code day 18 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 18.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let droplets = data.trim().split('\n')
		.map(|line| {
			let cods = line.split(',')
				.map(|x| x.parse::<isize>().expect("valid coords"))
				.collect::<Vec<isize>>();
			(cods[0], cods[1], cods[2])
		}).collect::<HashSet<(isize, isize, isize)>>();

	// Now, do our thing..
	let mut connections = 0;
	for drop in &droplets {
		let (drop_x, drop_y, drop_z) = drop;
		for (dx, dy, dz) in [
			(-1, 0, 0), (1, 0, 0),
			(0, -1, 0), (0, 1, 0),
			(0, 0, -1), (0, 0, 1),
		] {
			let side_drop = (drop_x+dx, drop_y+dy, drop_z+dz);
			if droplets.contains(&side_drop) {
				// We lose two sides
				connections += 1;
			}
		}
	}

	6 * droplets.len() - connections
}

/// Solve Advent of Code day 18 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 18.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	let droplets = data.trim().split('\n')
		.map(|line| {
			let cods = line.split(',')
				.map(|x| x.parse::<isize>().expect("valid coords"))
				.collect::<Vec<isize>>();
			(cods[0], cods[1], cods[2])
		}).collect::<HashSet<(isize, isize, isize)>>();
	let cod_ikks_min = *droplets.iter().map(|(x, _, _)| x).min().expect("min x");
	let cod_ikks_max = *droplets.iter().map(|(x, _, _)| x).max().expect("max x");
	let cod_way_min = *droplets.iter().map(|(_, y, _)| y).min().expect("min y");
	let cod_way_max = *droplets.iter().map(|(_, y, _)| y).max().expect("max y");
	let cod_zee_min = *droplets.iter().map(|(_, _, z)| z).min().expect("min z");
	let cod_zee_max = *droplets.iter().map(|(_, _, z)| z).max().expect("max z");

	let mut seen: HashSet<(isize, isize, isize)> = HashSet::new();
	let mut process: VecDeque<(isize, isize, isize)> = VecDeque::new();
	seen.insert((cod_ikks_min, cod_way_min-1, cod_zee_min));
	process.push_back((cod_ikks_min, cod_way_min-1, cod_zee_min));

	let mut sides = 0;
	while !process.is_empty() {
		// Pop one state
		let (here_x, here_y, here_z) = process.pop_front()
			.expect("at least one state remaining");
		// For all six around us..
		for (dx, dy, dz) in [
			(-1, 0, 0), (1, 0, 0),
			(0, -1, 0), (0, 1, 0),
			(0, 0, -1), (0, 0, 1),
		] {
			let there = (here_x+dx, here_y+dy, here_z+dz);
			// If this is a droplet, count it
			if droplets.contains(&there) { sides += 1; continue; }
			if (cod_ikks_min-1..=cod_ikks_max+1).contains(&there.0) &&
				(cod_way_min-1..=cod_way_max+1).contains(&there.1) &&
				(cod_zee_min-1..=cod_zee_max+1).contains(&there.2) {
				// It's free, accessible air
					if !seen.contains(&there) {
						seen.insert(there);
						process.push_back(there);
					}
				}
		}

	}

	sides
}

// vim: set tw=80:
