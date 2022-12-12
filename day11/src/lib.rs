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

use std::collections::{HashSet, VecDeque};

/// Solve Advent of Code day 11 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 11.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let mut monkeys = data.split("\n\n")
		.map(|st| st.parse::<Monkey>().unwrap())
		.collect::<Vec<Monkey>>();
	// We run 20 cycles
	for _ in 0..20 {
		for idx in 0..monkeys.len() {
			let base_monkey = monkeys.get_mut(idx).unwrap();
			let div = base_monkey.get_div();
			let out_monkeys = base_monkey.get_out_monkeys();
			let op = base_monkey.get_op();
			base_monkey.inspect_count_up();
			let items = base_monkey.get_values_mut();
			// Index of where stuff ends up
			let mut out_list: Vec<(usize, usize)> = Vec::new();
			for _ in 0..items.len() {
				let v = op.apply(items.pop_front().unwrap()) / 3;
				let out_monkey = if v % div == 0 {
					out_monkeys.0
				} else { out_monkeys.1 };
				out_list.push((out_monkey, v));
			}

			// Empty the list
			for (target, val) in out_list {
				monkeys.get_mut(target).unwrap()
					.get_values_mut().push_back(val);
			}
		}
		//println!("{:?}", monkeys);
	}
	let mut counts = monkeys.iter()
		.map(Monkey::get_inspect_count)
		.collect::<Vec<usize>>();
	counts.sort_unstable();
	counts.reverse();
	counts.first().unwrap() * counts.get(1).unwrap()
}

#[derive(Debug)]
struct Monkey {
	items: VecDeque<usize>,
	out_monkeys: (usize, usize),
	oper: Operation,
	cond: usize,
	inspect_count: usize,
}

#[derive(Debug,Copy,Clone)]
enum Operand {
	Val(usize),
	Old
}

#[derive(Debug,Copy,Clone)]
enum Operator {
	Add,
	Mul,
	Sub
}

#[derive(Debug,Copy,Clone)]
struct Operation(Operator, Operand);

impl Operation {
	const fn apply(&self, x: usize) -> usize {
		match self.1 {
			Operand::Old => {
				match self.0 {
					Operator::Add => x * 2,
					Operator::Sub => 0,
					Operator::Mul => x * x
				}
			},
			Operand::Val(v) => {
				match self.0 {
					Operator::Add => x + v,
					Operator::Sub => x - v,
					Operator::Mul => x * v
				}
			}
		}
	}
}

impl Monkey {
	fn get_values_mut(&mut self) -> &mut VecDeque<usize> {
		&mut self.items
	}

	fn inspect_count_up(&mut self) {
		self.inspect_count += self.items.len();
	}

	const fn get_inspect_count(&self) -> usize {
		self.inspect_count
	}

	const fn get_op(&self) -> Operation {
		self.oper
	}

	const fn get_div(&self) -> usize {
		self.cond
	}

	const fn get_out_monkeys(&self) -> (usize, usize) {
		self.out_monkeys
	}
}

impl std::str::FromStr for Monkey {
	type Err = ();
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		// Alright, let's suffer now
		let mut lines = st.split('\n').skip(1);
		// The order will always be the same
		let items: VecDeque<usize> = lines.next().unwrap()
			.split(": ").nth(1).unwrap()
			.split(", ")
			.map(|s| s.parse::<usize>().unwrap())
			.collect();
		// The operation ***sucks***
		let operation_str = lines.next().unwrap()
			.split("new = ").nth(1).unwrap();
		let mut operands = operation_str.split(' ').skip(1); // first one is
															 // always "old"
		let operator = match operands.next().unwrap() {
			"+" => Operator::Add,
			"-" => Operator::Sub,
			"*" => Operator::Mul,
			_ => unreachable!(),
		};
		let operand = match operands.next().unwrap() {
			"old" => Operand::Old,
			ns => {
				let num = ns.parse::<usize>().unwrap();
				Operand::Val(num)
			}
		};
		let operation = Operation(operator, operand);

		// Div factor
		let div_factor = lines.next().unwrap()
			.split(" by ").nth(1).unwrap()
			.parse::<usize>().unwrap();

		// Monkeys
		let monkey_true = lines.next().unwrap()
			.split("monkey ").nth(1).unwrap()
			.parse::<usize>().unwrap();
		let monkey_false = lines.next().unwrap()
			.split("monkey ").nth(1).unwrap()
			.parse::<usize>().unwrap();

		Ok(Self {
			out_monkeys: (monkey_true, monkey_false),
			items,
			oper: operation,
			cond: div_factor,
			inspect_count: 0,
		})
	}
}

/// Solve Advent of Code day 11 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 11.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	let mut monkeys = data.split("\n\n")
		.map(|st| st.parse::<Monkey>().unwrap())
		.collect::<Vec<Monkey>>();
	// So because we no longer divide by three, nothing prevents us from having
	// the numbers explode.
	// Well, nothing save for... modular arithmetic!
	// Since all of the modular division factors are prime, we can take the
	// multiple of all of them to figure out the smallest cycle of numbers we
	// need to keep track of

	let lcm: usize = monkeys.iter()
		.map(Monkey::get_div)
		.collect::<HashSet<usize>>()
		.iter()
		.product();

	// We run 10000 cycles
	for _ in 0..10000 {
		for idx in 0..monkeys.len() {
			let base_monkey = monkeys.get_mut(idx).unwrap();
			let div = base_monkey.get_div();
			let out_monkeys = base_monkey.get_out_monkeys();
			let op = base_monkey.get_op();
			base_monkey.inspect_count_up();
			let items = base_monkey.get_values_mut();
			// Index of where stuff ends up
			let mut out_list: Vec<(usize, usize)> = Vec::new();
			for _ in 0..items.len() {
				let v = op.apply(items.pop_front().unwrap()) % lcm;
				let out_monkey = if v % div == 0 {
					out_monkeys.0
				} else { out_monkeys.1 };
				out_list.push((out_monkey, v));
			}

			// Empty the list
			for (target, val) in out_list {
				monkeys.get_mut(target).unwrap()
					.get_values_mut().push_back(val);
			}
		}
		//println!("{:?}", monkeys);
	}
	let mut counts = monkeys.iter()
		.map(Monkey::get_inspect_count)
		.collect::<Vec<usize>>();
	counts.sort_unstable();
	counts.reverse();
	counts.first().unwrap() * counts.get(1).unwrap()
}

// vim: set tw=80:
