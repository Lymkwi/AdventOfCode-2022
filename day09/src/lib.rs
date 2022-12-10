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

/// Solve Advent of Code day 09 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 09.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	let mut model = RopeModel::new(1);
	data.split('\n')
		.map(|x| {
			let mut divs = x.split(' ');
			let mv = divs.next().unwrap().parse::<Move>().unwrap();
			let count = divs.next().unwrap().parse::<usize>().unwrap();
			(mv, count)
		})
		.for_each(|(mv, count)| model.apply_all(mv, count));
	model.tail_positions().iter().count()
}

#[derive(Debug, Copy, Clone)]
enum Move {
	Up,
	Down,
	Left,
	Right
}

impl std::str::FromStr for Move {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "U" {
			Ok(Self::Up)
		} else if s == "D" {
			Ok(Self::Down)
		} else if s == "L" {
			Ok(Self::Left)
		} else if s == "R" {
			Ok(Self::Right)
		} else {
			Err(())
		}
	}
}

struct RopeModel {
	//head: (isize, isize),
	//tail: (isize, isize),
	knots: Vec<(isize, isize)>,
	visited: HashSet<(isize, isize)>
}

impl RopeModel {
	fn new(knots: usize) -> Self {
		//Self { head: (0,0), tail: (0,0), visited: HashSet::new() }
		Self {
			knots: vec![(0,0); knots+1],
			visited: HashSet::new()
		}
	}

	fn apply(&mut self, mv: Move) {
		// Move the head
		let head = self.knots.get_mut(0).expect("Broken!");
		*head = match mv {
			Move::Up	=> (head.0, head.1+1),
			Move::Down	=> (head.0, head.1-1),
			Move::Left	=> (head.0-1, head.1),
			Move::Right	=> (head.0+1, head.1)
		};

		// Fix tail
		// if needed
		//println!("H: ({}, {})", self.head.0, self.head.1);
		//println!("D: {}", diff);
		// Scan through all of the ropes
		for idx in 0..self.knots.len()-1 {
			let head = self.knots.get(idx).expect("Head");
			let tail = self.knots.get(idx+1).expect("Tail");
			if isize::abs(head.0 - tail.0) > 1
				|| isize::abs(head.1 - tail.1) > 1 {
				// Fix vector
				let mut fix_vec = (head.0 - tail.0, head.1 - tail.1);
				// Now, make it only one move
				fix_vec = (fix_vec.0.signum(), fix_vec.1.signum());
				// Apply
				*self.knots.get_mut(idx+1).unwrap() = (tail.0 + fix_vec.0, tail.1 + fix_vec.1);
			}
		}

		//println!("T: ({}, {})", self.tail.0, self.tail.1);

		// Record tail position
		self.visited.insert(*self.knots.last().unwrap());
	}

	fn apply_all(&mut self, mv: Move, cnt: usize) {
		// Apply n times
		for _ in 0..cnt {
			self.apply(mv);
		}
	}

	fn tail_positions(&self) -> &HashSet<(isize, isize)> {
		&self.visited
	}
}

/// Solve Advent of Code day 09 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 09.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	let mut model = RopeModel::new(9);
	data.split('\n')
		.map(|x| {
			let mut divs = x.split(' ');
			let mv = divs.next().unwrap().parse::<Move>().unwrap();
			let count = divs.next().unwrap().parse::<usize>().unwrap();
			(mv, count)
		})
		.for_each(|(mv, count)| model.apply_all(mv, count));
	model.tail_positions().iter().count()
}

// vim: set tw=80:
