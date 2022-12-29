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

/// Solve Advent of Code day 17 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 17.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
	let orders = data.chars()
		.map(Jet::try_from)
		.collect::<Result<Vec<Jet>, String>>()
		.expect("Parsing error");

	let mut screen: Screen = Screen::new(orders);

	// Apply the stuff for one base cycle
	for _ in 0..2022 {
		screen.fall();
	}
	screen.get_height()
}

fn solve_cyclical(orders: Vec<Jet>, total: usize) -> usize {
	// So... where's the cycle?
	let order_cycle = orders.len();
	let minimal_cycle = (order_cycle * 5) / gcd(order_cycle, 5);
	// How many times does the cycle fit?
	let how_many_cycles = total / minimal_cycle;
	//println!("We determine the minimal cycle must be {minimal_cycle}");
	//println!("It will appear at most {how_many_cycles} times.");

	let mut screen: Screen = Screen::new(orders);

	// Apply the stuff for one base cycle
	for _ in 0..minimal_cycle {
		screen.fall();
	}
	let first_heights = screen.normalized_heights();
	//println!("After one minimal cycle, heights are {:?}", first_heights);
	let first_cycle_height = screen.get_height();
	//println!("The height is {first_cycle_height}");
	//println!("We felled {} pieces so far", screen.felled_pieces);

	// How many cycles will it take before we see that pattern again?
	let mut real_cycle: usize = 0;
	for a in 0..(how_many_cycles-1) {
		// Do a full cycle
		for _ in 0..minimal_cycle {
			screen.fall();
			//println!("{:?}", screen.normalized_heights());
		}
		// Get the heights
		let then_heights = screen.normalized_heights();
		if then_heights == first_heights {
			real_cycle = a+1;
			break;
		}
	}
	// Get the height for one full cycle
	let full_cycle_height = screen.get_height() - first_cycle_height;
	//println!("We find that the full cycle is actually {} times the smaller one",
	//		 real_cycle);
	//println!("That is to say, it is {} pieces", real_cycle * minimal_cycle);

	// Now, how many times does the full cycle appear?
	let how_many_full_cycles = (total - minimal_cycle) / (real_cycle * minimal_cycle);
	//println!("There will only be {how_many_full_cycles} full cycles");
	//println!("Each of which accounts for {full_cycle_height} of height");

	// What will be the remainder?
	let remainder = (total - minimal_cycle) % (real_cycle * minimal_cycle);
	//println!("The remainder will be {remainder}");
	for _ in 0..remainder {
		screen.fall();
	}
	// Get the remainder
	let remainder_height = screen.get_height() - (full_cycle_height + first_cycle_height);
	//println!("And adds {remainder_height} of height");

	remainder_height + first_cycle_height + full_cycle_height * how_many_full_cycles
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
	while a != 0 {
		let prev_a = a;
		a = b % a;
		b = prev_a;
	}
	b
}

struct Screen {
	heights: [usize; 7],
	next_piece: usize,
	jets: Vec<Jet>,
	jet_len: usize,
	jet_counter: usize,
	felled_pieces: usize,
	tetrominos: [Vec<(usize, usize)>; 5],
	memory: HashSet<(usize, usize)>
}

impl Screen {
	fn new(jets: Vec<Jet>) -> Self {
		let jlen = jets.len();
		Self {
			heights: [0; 7],
			next_piece: 0,
			jets,
			jet_len: jlen,
			jet_counter: 0,
			felled_pieces: 0,
			tetrominos: [
				vec![(0, 0), (0, 1), (0, 2), (0, 3)], // ####
				vec![
					/*   */ (2, 1), /*   */
					(1, 0), (1, 1), (1, 2),
					/*   */ (0, 1)  /*   */
				],
				vec![
					/*   */ /*   */ (2, 2),
					/*   */ /*   */ (1, 2),
					(0, 0), (0, 1), (0, 2),
				],
				vec![
					(3, 0),
					(2, 0),
					(1, 0),
					(0, 0)
				],
				vec![
					(1, 0), (1, 1),
					(0, 0), (0, 1)
				]
			],
			memory: HashSet::new(),
		}
	}

	/// Normalized heights that are memorized by the [`Screen`] structure
	fn normalized_heights(&self) -> [usize; 7] {
		// Get the heights and remove as much from them as possible
		let heights = self.heights;
		let min_height = heights.iter().min().unwrap();
		heights.into_iter()
			.map(|u| u - min_height)
			.collect::<Vec<usize>>()
			.try_into()
			.expect("7 width")
	}

	fn fall(&mut self) {
		// What height are we at?
		let height = self.get_height()+3;
		// Spawn the piece
		let mut rocks = self.tetrominos[self.next_piece]
			.iter()
			.copied()
			.map(|(y, x)| (y+height, x+2))
			.collect::<Vec<(usize, usize)>>();
		
		// Drop it by...
		loop {
			// Check the current jet order
			let cur_order = self.jets[self.jet_counter];
			// Change the jet counter...
			self.jet_counter += 1;
			self.jet_counter %= self.jet_len;
			// Check that the jet is applicable
			let mut jet_applicable = true;
			for rock in &rocks {
				if cur_order == Jet::Left && rock.1 == 0 {
					jet_applicable = false;
					break;
				}

				if cur_order == Jet::Right && rock.1 == 6 {
					jet_applicable = false;
					break;
				}

				let rock_applied = (rock.0, if cur_order == Jet::Left {
					rock.1 - 1
				} else {
					rock.1 + 1 // It will pass
				});

				if self.memory.contains(&rock_applied) {
					jet_applicable = false;
					break;
				}
			}

			// If applicable, apply the jet
			if jet_applicable {
				////println!("Applied jet {}",
						 //if cur_order == Jet::Left { "left" } else { "right" });
				for rock in &mut rocks {
					if cur_order == Jet::Left {
						rock.1 -= 1;
					} else {
						rock.1 += 1;
					}
				}
			}

			// Check whether we can go down
			let mut go_down = true;
			for rock in &rocks {
				if rock.0 == 0 {
					go_down = false;
					break;
				}

				let dropped_rock = (rock.0 - 1, rock.1);
				if self.memory.contains(&dropped_rock) {
					go_down = false;
					break;
				}
			}

			// If we can't, immobilize
			if !go_down {
				for rock in &rocks {
					let (height, x) = rock;
					// Change heights
					self.heights[*x] = self.heights[*x].max(height+1);
					// Memorize
					//assert!(!self.memory.contains(&rock));
					self.memory.insert(*rock);
				}
				//
				////println!("Added {rocks:?}");
				////println!("heights are now {:?}\n", self.heights);
				// Change the tetromino counter...
				self.next_piece += 1;
				self.next_piece %= 5;
				// Count
				self.felled_pieces += 1;
				// Return
				return;
			}
			// if we can
			// Apply down
			rocks.iter_mut()
				.for_each(|rock| rock.0 -= 1);
			////println!("Standing {rocks:?}");
		}
	}

	fn get_height(&self) -> usize {
		*self.heights.iter().max().unwrap()
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Jet {
	Left,
	Right,
}

impl TryFrom<char> for Jet {
	type Error = String;
	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'<' => Ok(Self::Left),
			'>' => Ok(Self::Right),
			x => Err(format!("Unknown order character '{x}'"))
		}
	}
}

/// Solve Advent of Code day 17 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 17.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	let orders = data.chars()
		.map(Jet::try_from)
		.collect::<Result<Vec<Jet>, String>>()
		.expect("Parsing error");

	// Apply the stuff it's pretty straightforward
	solve_cyclical(orders, 1_000_000_000_000_usize)
}

// vim: set tw=80:
