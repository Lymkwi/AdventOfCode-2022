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

use std::collections::HashMap;

/// Solve Advent of Code day 10 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds today's input
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 10.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	let mut cpu = Device::new(40, 6, false);

	data.split('\n')
		.for_each(|s| {
			let mut splits = s.split(' ');
			let op = splits.next().unwrap();
			if op == "noop" {
				cpu.pass_cycle();
			} else {
				let count = splits.next().unwrap().parse::<isize>().unwrap();
				cpu.addx(count);
			}
		});

	cpu.spit()
}

struct Device {
	every: isize,
	howmany: isize,
	cyclecount: isize,
	value: isize,
	screen: HashMap<(isize, isize), bool>,
	record: bool,
	strengths: Vec<isize>,
	start: isize
}

impl Device {
	fn new(every: isize, howmany: isize, record: bool) -> Self {
		Self {
			start: 20, every, howmany, cyclecount: 0, value: 1,
			strengths: Vec::new(), record, screen: HashMap::new()
		}
	}

	fn update_screen(&mut self) {
		// Count the pixel on screen (maybe)
		let pixelbeam = self.cyclecount % 40;
		for dx in [-1, 0, 1] {
			if self.value + dx == pixelbeam {
				self.screen.insert((self.cyclecount / 40, self.value + dx), true);
			}
		}

	}

	fn pass_cycle(&mut self) {
		self.cyclecount += 1;
		if self.record || self.cyclecount < self.start { return; }
		if (self.cyclecount - self.start) % self.every == 0 && self.strengths.len() <= self.howmany.try_into().unwrap() {
			self.strengths.push(self.value * self.cyclecount);
		}
	}

	fn addx(&mut self, howmuch: isize) {
		self.pass_cycle();
		if self.record { self.update_screen(); }
		self.pass_cycle();
		self.value += howmuch;
		if self.record { self.update_screen(); }
	}

	fn spit(&self) -> usize {
		self.strengths.iter().sum::<isize>().try_into().unwrap()
	}

	const fn get_screen(&self) -> &HashMap<(isize, isize), bool> {
		&self.screen
	}
}

/// Solve Advent of Code day 10 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 10.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> String {
	let mut cpu = Device::new(20, 20, true);
	cpu.update_screen();

	data.split('\n')
		.for_each(|s| {
			let mut splits = s.split(' ');
			let op = splits.next().unwrap();
			if op == "noop" {
				cpu.pass_cycle();
				cpu.update_screen();
			} else {
				let count = splits.next().unwrap().parse::<isize>().unwrap();
				cpu.addx(count);
			}

		});

	print_map(cpu.get_screen())
}

fn print_map(sc: &HashMap<(isize, isize), bool>) -> String {
	(0..6_isize).map(|y: isize| {
		(0..40_isize).map(|x| {
			if *sc.get(&(y,x)).unwrap_or(&false) { "#" } else { "." }
		}).collect::<Vec<&str>>().as_slice().join("")
	}).collect::<Vec<String>>().as_slice().join("\n")
}

