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

// Some stuff allowed today
#![allow(clippy::cast_possible_wrap)]

use std::collections::HashMap;
use std::str::FromStr;

/// Solve Advent of Code day 08 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 08.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let mut grid = Grid::from_str(data).expect("Failed");
	let grid_width = grid.get_width();
	let grid_height = grid.get_height();
	for xpos in 1..grid_height-1 {
		for ypos in 1..grid_width-1 {
			let (my_height, _, _) = grid.at(ypos, xpos);
			// In all four directions, go up/down/left/down until we find either
			// a visible edge, a visible smaller than us, or a hidden
			let mut seen = false;
			let mut whence = 0;

			for (ydiff, xdiff, xcode) in [(-1, 0, 0b0001), (0, -1, 0b0010), (1, 0, 0b0100), (0, 1, 0b1000)] {
				let mut ny = ypos + ydiff;
				let mut nx = xpos + xdiff;
				loop {
					let (height, status, visfrom) = grid.at(ny, nx);
					if let Some(visible) = status {
						if visfrom == xcode && visible && my_height > height{
							// We're visible
							seen = true;
							whence = xcode;
							break;
						}
					}
					if height >= my_height {
						// Break, we're not going to get seen from there
						//println!("({}, {}) hit a wall for ({}, {})", ypos, xpos, ydiff, xdiff);
						break;
					}
					if ny == 0 || nx == 0 || ny == grid_width-1 || nx == grid_height-1 {
						//println!("({}, {}) hit a wall for ({}, {})", ypos, xpos, ydiff, xdiff);
						seen = true;
						whence = xcode;
						break; 
					}
					ny += ydiff;
					nx += xdiff;
				}
				if seen { grid.visible(ypos, xpos, whence); break; }
			}

			if !seen {
				grid.hide(ypos, xpos);
			}
		}
	}
	//println!("{}", grid);
	grid.count_visible()
}

#[derive(Debug)]
struct Grid {
	grid: HashMap<(isize, isize), (usize, Option<bool>, usize)>,
	width: isize,
	height: isize
}

impl Grid {
	const fn get_width(&self) -> isize {
		self.width
	}

	const fn get_height(&self) -> isize {
		self.height
	}

	fn at(&self, y_pos: isize, x_pos: isize) -> (usize, Option<bool>, usize) {
		self.grid[&(y_pos, x_pos)]
	}

	fn hide(&mut self, y_pos: isize, x_pos: isize) {
		self.grid.entry((y_pos, x_pos))
			.and_modify(|(_, status, _)| *status = Some(false) );
	}

	fn visible(&mut self, y_pos: isize, x_pos: isize, dir: usize) {
		self.grid.entry((y_pos, x_pos))
			.and_modify(|(_, status, mydir)| { *status = Some(true); *mydir = dir; } );
	}

	fn count_visible(&self) -> usize {
		self.grid.values()
			.filter(|(_, v, _)| v.map_or(false, |k| k))
			.count()
	}
}

impl std::fmt::Display for Grid {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for ypos in 0..self.height {
			for xpos in 0..self.width {
				let (height, vis, _) = self.at(ypos, xpos);
				if let Some(visible) = vis {
					if visible {
						write!(fmt, "{}", height)?;
					} else {
						write!(fmt, " ")?;
					}
				} else {
					write!(fmt, ".")?;
				}
			}
			writeln!(fmt)?;
		};
		Ok(())
	}
}

impl FromStr for Grid {
	type Err = ();
	fn from_str(data: &str) -> Result<Self, Self::Err> {
		let lines = data.trim().split('\n').collect::<Vec<&str>>();
		let columns = lines[0].len();
		let rows = lines.len();

		let grid = lines.iter()
			.enumerate()
			.flat_map(|(u, v)| {
				v.chars()
					.enumerate()
					.map(|(n, x)| {
						let status = if u == 0 || n == 0 || u == rows-1 || n == columns-1 { Some(true) } else { None };
						((u as isize, n as isize), (x.to_digit(10).unwrap() as usize, status, 0))
					})
					.collect::<Vec<((isize, isize), (usize, Option<bool>, usize))>>()
			})
			.collect::<HashMap<_, _>>();

		Ok(Self { grid, width: columns as isize, height: rows as isize })
	}
}

/// Solve Advent of Code day 08 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 08.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	let grid = Grid::from_str(data).expect("Failed");
	let grid_width = grid.get_width();
	let grid_height = grid.get_height();

	let mut scores: HashMap<(isize, isize), usize> = HashMap::new();

	for ypos in 1..grid_height-1 {
		for xpos in 1..grid_width-1 {
			let (my_height, _, _) = grid.at(ypos, xpos);

			let scenery_score = [
					(-1, 0), (0, -1),
					(0, 1), (1, 0)
				].iter()
				.map(|(ydiff, xdiff)| {
					let mut ny = ypos + ydiff;
					let mut nx = xpos + xdiff;
					let mut count = 0;

					while ny >= 0 && nx >= 0 && ny < grid_height && nx < grid_width {
						// Count
						let (height, _, _) = grid.at(ny, nx);
						count += 1;
						if height >= my_height {
							break;
						}

						ny += ydiff;
						nx += xdiff;
					}
					//println!("({}, {}) breaks ({}, {}) with {}",
								//ypos, xpos, ydiff, xdiff, count);
					count
				})
				.fold(1, |s, v| v * s);
			scores.insert((ypos, xpos), scenery_score);
		}
	}

	*scores.values().max().unwrap()
}

// vim: set tw=80:
