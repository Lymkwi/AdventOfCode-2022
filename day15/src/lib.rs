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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref LINERGX: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

/// Solve Advent of Code day 15 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 15.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let sensors = data.trim().split('\n')
		.map(|line| {
			let caps = LINERGX.captures(line).expect("Regex works");
			let sens_x = caps[1].parse::<isize>().expect("sensor x");
			let sens_y = caps[2].parse::<isize>().expect("sensor y");
			let baco_x = caps[3].parse::<isize>().expect("beacon x");
			let baco_y = caps[4].parse::<isize>().expect("beacon y");
			Sensor {
				x: sens_x, y: sens_y,
				radius: distance((sens_y, sens_x), (baco_y, baco_x))
			}
		})
		.collect::<Vec<Sensor>>();
	let bacons = data.trim().split('\n')
		.map(|line| {
			let caps = LINERGX.captures(line).expect("Regex works");
			let baco_x = caps[3].parse::<isize>().expect("beacon x");
			let baco_y = caps[4].parse::<isize>().expect("beacon y");
			(baco_y, baco_x)
		})
		.collect::<HashSet<(isize, isize)>>();
	let minx = sensors.iter()
		.map(|s| s.x - s.radius)
		.min().expect("More than 0 sensors");
	let maxx = sensors.iter()
		.map(|s| s.x + s.radius)
		.max().expect("More than 0 sensors");

	let mut count = 0;
	let y = 2_000_000;
	for x in minx..=maxx {
		let cods = (y, x);
		// For every single one of these...
		if sensors.iter()
			.any(|s| distance(cods, (s.y, s.x)) <= s.radius) &&
			!bacons.contains(&cods) {
			count += 1;
		}
	}

	count
}

#[derive(Debug)]
struct Sensor {
	y: isize,
	x: isize,
	radius: isize
}

const fn distance(a: (isize, isize), b: (isize, isize)) -> isize {
	isize::abs(a.0 - b.0) + isize::abs(a.1 - b.1)
}

/// Solve Advent of Code day 15 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 15.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	let sensors = data.trim().split('\n')
		.map(|line| {
			let caps = LINERGX.captures(line).expect("Regex works");
			let sens_x = caps[1].parse::<isize>().expect("sensor x");
			let sens_y = caps[2].parse::<isize>().expect("sensor y");
			let baco_x = caps[3].parse::<isize>().expect("beacon x");
			let baco_y = caps[4].parse::<isize>().expect("beacon y");
			Sensor {
				x: sens_x, y: sens_y,
				radius: distance((sens_y, sens_x), (baco_y, baco_x))
			}
		})
		.collect::<Vec<Sensor>>();

	// Find all of the positions that are just one outside of a sensor field
	// By our logic, if there exists only one position outside of all fields in
	// the designated area, it is necessarily one tile away from a field.
	let edge_cases = sensors.iter()
		.flat_map(|s| {
			// Alright, start collecting
			// Thankfully, Manhatthan distance makes big losange shapes around
			// the sensors, so it's easy to walk around them
			let mut poses = Vec::new();
			for delta in 0..=s.radius {
				// Start at the left edge, and go up
				poses.push((s.y - delta, s.x - s.radius - 1 + delta));
				// Now from up above to the right
				poses.push((s.y - s.radius - 1 - delta, s.x + delta));
				// From the right to the bottom
				poses.push((s.y + delta, s.x + s.radius + 1 - delta));
				// And now right to left
				poses.push((s.y + s.radius + 1 + delta, s.x - delta));
			}
			poses
		})
		.filter(|&(y, x)| (0..=4_000_000).contains(&y) && (0..=4_000_000).contains(&x))
		//.filter(|&(y, x)| 0 <= y && y <= 20 && 0 <= x && x <= 20)
		// For each of them, filter out those that are in a zone
		.filter(|&cods| sensors.iter().all(|s| distance(cods, (s.y, s.x)) > s.radius))
		.collect::<HashSet<(isize, isize)>>();

	// Now, how many are there?

	assert_eq!(1, edge_cases.len());

	let solution = edge_cases.iter().next().expect("At least one solution");
	usize::try_from(solution.1 * 4_000_000 + solution.0).expect("Within range")
}

// vim: set tw=80:
