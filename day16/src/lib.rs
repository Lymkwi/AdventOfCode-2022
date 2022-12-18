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
	cmp::Ordering,
	collections::{HashMap, HashSet, BinaryHeap}
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref LINE: Regex = Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); (tunnel leads|tunnels lead) to (valve|valves) ([A-Z, ]*)$").unwrap();
}

/// Solve Advent of Code day 16 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 16.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	let maze = data.trim().split('\n')
		.map(|line| {
			let caps = LINE.captures(line).unwrap();
			(
				String::from(&caps[1]),
				(caps[2].parse::<usize>().unwrap(),
				caps[5].split(", ").map(String::from).collect::<Vec<String>>())
			)
		}).collect::<HashMap<String, (usize, Vec<String>)>>();

	// So, first, reduce this fucking map because it's unbearable to have so
	// many f*cking empty nodes
	let mut non_null_valves = maze.iter()
		.filter_map(|(valve, (press, _))| {
			if *press > 0 { Some(valve.as_str()) } else { None }
		})
		.collect::<Vec<&str>>();
	non_null_valves.push("AA"); // We need AA because we start from there

	let mut distances: HashMap<String, HashMap<String, usize>> = HashMap::new();
	for valve in &non_null_valves {
		// Ew
		distances.insert((*valve).to_string(), HashMap::new());
	}
	for valve_one in &non_null_valves {
		for valve_two in &non_null_valves {
			// Ew
			if valve_one == valve_two { continue; }
			let valve_one = (*valve_one).to_string();
			let valve_two = (*valve_two).to_string();
			let dist = distance(&maze, &valve_one, &valve_two);
			distances.get_mut(&valve_one).expect("fetchable")
					.insert(valve_two.clone(), dist);
			distances.get_mut(&valve_two).expect("fetchable")
					.insert(valve_one.clone(), dist);
		}
	}

	//let mut memo: HashMap<(String, usize, Vec<String>), usize> = HashMap::new();

	// Let's start backwards
	//let memo: HashMap<(String, usize), (usize, Vec<String>)> = HashMap::new();

	let mut states: BinaryHeap<MazeState> = BinaryHeap::new();
	let mut seen: HashSet<MazeState> = HashSet::new();
	let mut best: usize = 0;

	let max_power: usize = maze.values().map(|(p, _)| p).sum();

	states.push(MazeState::default());

	while !states.is_empty() {
		let state = states.pop().expect("At least one state");
		let pressure = state.pressure;
		let minutes = state.minutes;
		if minutes == 0 {
			if pressure > best {
				best = best.max(pressure);
				println!("{state:?}");
				println!("Found max {best} ({} states)", states.len());
			}
			continue;
		}
		let power = state.power;
		if pressure + max_power * minutes <= best {
			// Don't bother
			continue;
		}
		let open = state.open.clone();
		let current = state.current;
	
		let press = maze[&current].0;
	
		// Maybe open?
		if !open.contains(&current) {
			// press cannot be zero
			// Spend a minute
			let minutes = minutes - 1;
			let pressure = pressure + power;
			let power = power + press;
			let mut new_open = open.clone(); new_open.push(current.clone());
			let new_state = MazeState::build(minutes, pressure, current.clone(), power, new_open);
			if !seen.contains(&new_state) {
				seen.insert(new_state.clone());
				states.push(new_state);
			}
		}

		// Where can we go
		for (target, cost) in &distances[&current] {
			// This will re-add "AA" as a state we can get to but it's always
			// either a waste of time or a necessary step to the right output
			// so...
			if *cost > minutes {
				// no need
				continue;
			}
			let pressure = pressure + cost * power;
			let new_state = MazeState::build(minutes-cost, pressure, target.clone(), power, open.clone());
			if !seen.contains(&new_state) {
				seen.insert(new_state.clone());
				states.push(new_state);
			}
		}

		// What if we just... stopped?
		let pressure = pressure + minutes * power;
		let new_state = MazeState::build(0, pressure, current, power, open.clone());
		if !seen.contains(&new_state) {
			seen.insert(new_state.clone());
			states.push(new_state);
		}
	}

	best
}

fn distance(maze: &HashMap<String, (usize, Vec<String>)>, a: &str, b: &str) -> usize {
	// Try and find the closest path between these two..
	if a == b {
		return 0;
	}
	let mut states: BinaryHeap<(usize, String)> = BinaryHeap::new();
	let mut visited: HashSet<String> = HashSet::new();
	states.push((0, String::from(a)));
	while !states.is_empty() {
		let (cost, point) = states.pop().expect("At least one node");
		let conns = &maze[&point].1;
		for c in conns {
			if c == b {
				return cost + 1;
			}
			if !visited.contains(c) {
				states.push((cost+1, c.clone()));
				visited.insert(c.clone());
			}
		}
	}
	panic!("Not reached");
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct MazeState {
	minutes: usize,
	pressure: usize,
	power: usize,
	current: String,
	open: Vec<String>,
	//orders: String, // that was for debugging
}

impl MazeState {
	fn build(minutes: usize, pressure: usize, current: String, power: usize, open: Vec<String>) -> Self {
		Self { minutes, pressure, power, current, open }
	}
}

impl Default for MazeState {
	fn default() -> Self {
		Self {
			minutes: 30,
			pressure: 0,
			current: String::from("AA"),
			power: 0, open: Vec::new(),
			//orders: String::from("Start from AA")
		}
	}
}

impl PartialOrd for MazeState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let mine = self.minutes * self.power + self.pressure;
		let theirs = other.minutes * other.power + other.pressure;
		mine.partial_cmp(&theirs)
	}
}

impl Ord for MazeState {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("usize is Ord")
	}
}

/// Solve Advent of Code day 16 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 16.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	0
}

// vim: set tw=80:
