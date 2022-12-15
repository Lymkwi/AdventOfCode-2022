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

use std::collections::{HashMap, HashSet};

/// Solve Advent of Code day 14 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 14.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let mut states: HashMap<(isize, isize), SandState> = HashMap::new();
	// Trace the paths
	let blocks = trace_paths(data);
	let mut num_sand = 0;
	let floor_y = blocks.iter().map(|(y, _)| *y).max().expect("Floor Y");
	loop {
		//print_grid(&blocks, &states);
		// Drop the sand
		// Will return true if sand dropped below floor level
		if drop_sand(floor_y, &blocks, &mut states).1 {
			return num_sand;
		}
		num_sand += 1;
	}
}

fn drop_sand(floor: isize,
			 grid: &HashSet<(isize, isize)>,
			 states: &mut HashMap<(isize, isize), SandState>
) -> ((isize, isize), bool) {
	// Now, try and drop one unit
	let mut cods = (0, 500);
	while cods.0 < floor {
		// Look at the current state map
		// Depending on the state, take actions
		match states.get(&cods).unwrap_or(&SandState::GoDown) {
			SandState::GoDown => {
				let down = (cods.0+1, cods.1);
				if grid.contains(&down) || *states.get(&down)
					.unwrap_or(&SandState::GoDown) == SandState::Still {
						// Uhm, let's not go down
						// Change the current state
						states.insert(cods, SandState::GoLeft);
					} else {
						// Move
						states.insert(cods, SandState::GoDown);
						cods = down;
					}
			},
			SandState::GoLeft => {
				let left = (cods.0+1, cods.1-1);
				if grid.contains(&left) || *states.get(&left)
					.unwrap_or(&SandState::GoDown) == SandState::Still {
						states.insert(cods, SandState::GoRight);
					} else {
						cods = left;
					}
			},
			SandState::GoRight => {
				let right = (cods.0+1, cods.1+1);
				if grid.contains(&right) || *states.get(&right)
					.unwrap_or(&SandState::GoDown) == SandState::Still {
						states.insert(cods, SandState::GoStill);
					} else {
						cods = right;
					}
			},
			SandState::GoStill => {
				states.insert(cods, SandState::Still);
				return (cods, false);
			},
			SandState::Still => unreachable!()
		}
	}
	(cods, true)
}

fn trace_paths(data: &str) -> HashSet<(isize, isize)> {
	// Split the thing into paths
	data.trim().split('\n')
		.flat_map(|path| {
			let nodes = path.split(" -> ")
				.map(|end| {
					let nums = end.split(',')
						.map(|x| x.parse::<isize>().unwrap())
						.collect::<Vec<isize>>();
					// The problem uses x,y coordinates
					(nums[1], nums[0])
				})
				.collect::<Vec<(isize, isize)>>();
			nodes[..].windows(2)
				.flat_map(|c| {
					let (y_1, x_1) = c[0];
					let (y_2, x_2) = c[1];

					if y_2 == y_1 {
						let x_min = x_1.min(x_2);
						let x_max = x_2.max(x_1);
						(x_min..=x_max)
							.map(|v| (y_1, v))
							.collect::<Vec<(isize, isize)>>()
					} else {
						let y_min = y_1.min(y_2);
						let y_max = y_1.max(y_2);
						(y_min..=y_max)
							.map(|v| (v, x_1))
							.collect()
					}
				})
				.collect::<Vec<(isize, isize)>>()
		})
		.collect::<HashSet<(isize, isize)>>()
}

/*fn print_grid(grid: &HashSet<(isize, isize)>, states: &HashMap<(isize, isize), SandState>) {
	let mut whyes = grid.iter().map(|(y, _)| *y).collect::<Vec<isize>>();
	let mut ekkses = grid.iter().map(|(_, x)| *x).collect::<Vec<isize>>();
	whyes.sort_unstable();
	ekkses.sort_unstable();
	let y_min: isize = *whyes.first().expect("Y minimum").min(&0);
	let y_max: isize = *whyes.last().expect("Y maximum").max(&0);
	let x_min: isize = *ekkses.first().expect("X minimum").min(&500);
	let x_max: isize = *ekkses.last().expect("X maximum").max(&500);

	for y in y_min..=y_max {
		for x in x_min..=x_max {
			if y == 0 && x == 500 {
				print!("X");
			} else if grid.contains(&(y, x)) {
				print!("#");
			} else if let Some(s) = states.get(&(y, x)) {
				print!("{}", match s {
					SandState::GoDown => "v",
					SandState::GoLeft => "/",
					SandState::GoRight => "\\",
					SandState::GoStill => "!",
					SandState::Still => "o"
				});
			} else {
				print!("⋅");
			}
		}
		println!();
	}
}*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SandState {
	Still,
	GoDown,
	GoLeft,
	GoRight,
	GoStill,
}

/// Solve Advent of Code day 14 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 14.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	let mut states: HashMap<(isize, isize), SandState> = HashMap::new();
	// Trace the paths
	let mut blocks = trace_paths(data);
	let mut num_sand = 1;
	let floor_y = blocks.iter().map(|(y, _)| *y).max().expect("Floor Y");
	let mut ekkses = blocks.iter().map(|(_, x)| *x).collect::<Vec<isize>>();
	ekkses.sort_unstable();
	// Add the bottom
	for x in 500-(floor_y+3)..=(500+floor_y+3) {
		blocks.insert((floor_y+2, x));
	}
	loop {
		// Drop the sand
		// Will return true if sand dropped below floor level
		let cods = drop_sand(floor_y+2, &blocks, &mut states).0;
		if cods == (0, 500) {
			//print_grid(&blocks, &states);
			return num_sand;
		}
		num_sand += 1;
	}
}

// vim: set tw=80:
