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

/// Solve Advent of Code day 05 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `String`, the result for part one of advent of code
/// day 05. It contains the series of letters marking the crates that end up on
/// top of each stack
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> String {
	// Parsing is hell today!
	let parts_boundary = data.find("\n\n").expect("Invalid input!");
	let mut stacks = parse_diagram(data[..parts_boundary].trim_end());

	let orders = &data[parts_boundary+2..];
	for order in parse_orders(orders) {
		let (count, src, dst) = order;
		let (firstsplit, secondsplit) = stacks.split_at_mut((src-1).max(dst-1));
		let (srcqueue, dstqueue) = if src < dst {
			(&mut firstsplit[src-1], &mut secondsplit[0])
		} else {
			(&mut secondsplit[0], &mut firstsplit[dst-1])
		};
		for _ in 0..count {
			dstqueue.push(srcqueue.pop().expect("Incident"));
		}
		//println!("{:?}", stacks);
	}
	stacks.iter()
		.map(|s| s.last().unwrap())
		.collect()
}

/// Parse the diagram part of the input
///
/// That is, from the argument given, which is the part of the input that
/// contains only the diagram, return a vector of vectors of letters that
/// contain the crates in the correct order.
fn parse_diagram(diagram: &str) -> Vec<Vec<char>> {
	// Step one, find how many goddamn columns there
	let lines = diagram.split('\n').collect::<Vec<_>>();
	let cargo_count = lines.last().expect("Invalid input!")
		.split(' ')
		.last().expect("Invalid input!")
		.parse().expect("Invalid column count!");
	let mut line_vector = Vec::with_capacity(cargo_count);
	// Create a vector for each
	for _ in 0..cargo_count {
		line_vector.push(Vec::new());
	}

	// Part 2, fill the vectors
	for line in &lines[0..lines.len()-1] {
		for idx in line.match_indices('[') {
			let v: char = line.chars().nth(idx.0+1).unwrap();
			let column = idx.0/4;
			line_vector[column].insert(0, v);
		}
	}
	line_vector
}

/// Parse the orders from the day's input
///
/// All orders have the same shape:
/// `move X from Y to Z`, where `X` is a positive strict amount and `Y` and `Z`
/// are column numbers.
fn parse_orders(orders: &str) -> Vec<(usize, usize, usize)> {
	orders.trim().split('\n')
		.map(|line| {
			let count: usize = line[5..].split(' ')
				.next().unwrap()
				.parse().unwrap();
			let source: usize = line[line.find("from ").unwrap()+5..].split(' ')
				.next().unwrap()
				.parse().unwrap();
			let dest: usize = line[line.find("to ").unwrap()+3..].split(' ')
				.next().unwrap()
				.parse().unwrap();
			(count, source, dest)
		})
		.collect()
}

/// Solve Advent of Code day 05 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `String`, the result for part
/// two of advent of code day 05. This time, we can grab more than one crate at
/// a time!
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> String {
	// Parsing is hell today!
	let parts_boundary = data.find("\n\n").expect("Invalid input!");
	let mut stacks = parse_diagram(data[..parts_boundary].trim_end());

	let orders = &data[parts_boundary+2..];
	for order in parse_orders(orders) {
		let (count, src, dst) = order;
		let (firstsplit, secondsplit) = stacks.split_at_mut((src-1).max(dst-1));
		let (srcqueue, dstqueue) = if src < dst {
			(&mut firstsplit[src-1], &mut secondsplit[0])
		} else {
			(&mut secondsplit[0], &mut firstsplit[dst-1])
		};
		// The fun thing about grabbing all crates at once, right,
		// is that it's like grabbing one by one to a temporary column,
		// and then grabbing them again
		// Highly inefficient, but changes code minimally
		let mut bufferpile: Vec<char> = Vec::with_capacity(count);
		for _ in 0..count {
			bufferpile.push(srcqueue.pop().expect("Incident"));
		}
		while !bufferpile.is_empty() {
			dstqueue.push(bufferpile.pop().expect("Incident 2!"));
		}
	}
	stacks.iter()
		.map(|s| s.last().unwrap())
		.collect()
}

