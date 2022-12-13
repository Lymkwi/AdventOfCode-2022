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
	cmp::Reverse,
	collections::{
		BinaryHeap,
		HashMap,
		HashSet,
	},
	str::FromStr,
};

/// Solve Advent of Code day 12 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 12.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let map = XMap::from_str(data).expect("Unable to build map");
	map.smallest_path(false)
}

struct XMap {
	start: (isize, isize),
	end: (isize, isize),
	grid: HashMap<(isize, isize), u32>
}

impl XMap {
	fn smallest_path(&self, part2: bool) -> usize {
		// We're using Djikstra's algorithm
		let mut heap: BinaryHeap<Reverse<(usize, (isize, isize))>> = BinaryHeap::new();
		let mut visited = HashSet::new();
		if part2 {
			self.grid.iter()
				.filter_map(|(coords, &val)| if val == 97 { Some(coords) } else { None })
				.for_each(|coords| {
					heap.push(Reverse((0, *coords)));
					visited.insert(*coords);
				});
		} else {
			heap.push(Reverse((0, self.start)));
			visited.insert(self.start);
		}
		while !heap.is_empty() {
			// Take one
			let Reverse((cost, coords)) = heap.pop().unwrap();
			// If we're at the end, return
			if coords == self.end {
				return cost;
			}
			// Get the cost at our location
			let here = self.grid[&coords];
			// Add one for each direction
			for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
				let ny = dy + coords.0;
				let nx = dx + coords.1;

				if visited.contains(&(ny, nx)) {
					continue;
				}
				if let Some(&there) = self.grid.get(&(ny, nx)) {
					if here + 1 >= there {
						visited.insert((ny, nx));
						heap.push(Reverse((cost+1, (ny, nx))));
					}
				}
			}
		}
		panic!("No solution found!");
	}
}

impl FromStr for XMap {
	type Err = ();
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		let mut start = (0, 0);
		let mut end = (0, 0);
		let mut grid = HashMap::new();

		for (row, line) in st.split('\n').enumerate() {
			let row: isize = row.try_into().unwrap();
			for (col, chr) in line.chars().enumerate() {
				let col: isize = col.try_into().unwrap();
				if chr == 'S' {
					start = (col, row);
					grid.insert((col, row), 97); // 'a'
				} else if chr == 'E' {
					end = (col, row);
					grid.insert((col, row), 122); // 'z'
				} else {
					let chr: u32 = chr.try_into().unwrap();
					grid.insert((col, row), chr);
				}
			}
		}

		Ok(Self { start, end, grid })
	}
}

/// Solve Advent of Code day 12 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 12.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	data.parse::<XMap>().unwrap().smallest_path(true)
}

// vim: set tw=80:
