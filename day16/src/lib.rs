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
	cmp::{Ordering, Reverse},
	collections::{HashMap, HashSet, BinaryHeap}
};

use itertools::Itertools;
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
	// Djikstra to establish initial distances used for everything else
	for valve_one in &non_null_valves {
		for valve_two in &non_null_valves {
			// Ew
			if valve_one == valve_two { continue; }
			let valve_one = (*valve_one).to_string();
			let valve_two = (*valve_two).to_string();
			let dist = distance(&maze, &valve_one, &valve_two);
			distances.get_mut(&valve_one).expect("fetchable")
					.insert(valve_two.clone(), dist);
			//distances.get_mut(&valve_two).expect("fetchable")
					//.insert(valve_one.clone(), dist);
			assert_eq!(distance(&maze, &valve_one, &valve_two), distance(&maze, &valve_two, &valve_one));
		}
	}

	compute(&maze, &distances, None, 30, Vec::new(), false).0
}

fn distance(maze: &HashMap<String, (usize, Vec<String>)>, a: &str, b: &str) -> usize {
	// Try and find the closest path between these two..
	if a == b {
		return 0;
	}
	let mut states: BinaryHeap<Reverse<(usize, String)>> = BinaryHeap::new();
	let mut visited: HashSet<String> = HashSet::new();
	states.push(Reverse((0, String::from(a))));
	while !states.is_empty() {
		let Reverse((cost, point)) = states.pop().expect("At least one node");
		let conns = &maze[&point].1;
		for c in conns {
			if c == b {
				return cost+1;
			}
			if !visited.contains(c) {
				states.push(Reverse((cost+1, c.clone())));
				visited.insert(c.clone());
			}
		}
	}
	panic!("Not reached");
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct MazeState<'a> {
	minutes: usize,
	pressure: usize,
	power: usize,
	current: String,
	open: Vec<&'a str>,
	//orders: String, // that was for debugging
}

impl<'mz> MazeState<'mz> {
	fn build(minutes: usize, pressure: usize, /*orders: String,*/ current: String, power: usize, open: Vec<&'mz str>) -> Self {
		Self { minutes, pressure, power, current, open}
	}
}

impl<'mz> Default for MazeState<'mz> {
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

impl PartialOrd for MazeState<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let mine = self.minutes * self.power + self.pressure;
		let theirs = other.minutes * other.power + other.pressure;
		mine.partial_cmp(&theirs)
	}
}

impl Ord for MazeState<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("usize is Ord")
	}
}

fn compute<'a>(
	maze: &HashMap<String, (usize, Vec<String>)>,
	distances: &'a HashMap<String, HashMap<String, usize>>,
	early_cut: Option<usize>, // score that you need to be able to theoretically
							  // reach in order to be viable
	start_minutes: usize,
	open: Vec<&'a str>,
	memorize: bool
) -> (usize, Option<HashSet<MazeState<'a>>>) {
	// This is all of the solving code
	let mut best: usize = 0;

	let max_power: usize = maze.iter()
		.filter_map(|(n, (p, _))|
					if open.contains(&n.as_str()) {
						None
					} else {
						Some(p)
					}).sum();
	if let Some(cut) = early_cut {
		if max_power * start_minutes <= cut {
			// No need to bother
			return (0, None);
		}
	}
	let mut states: BinaryHeap<MazeState> = BinaryHeap::new();
	let mut seen: HashSet<MazeState> = HashSet::new();
	let mut best_states: HashSet<MazeState> = HashSet::new();

	states.push(MazeState::build(start_minutes, 0, "AA".into(), 0,
		open));

	while !states.is_empty() {
		let state = states.pop().expect("At least one state");
		let pressure = state.pressure;
		let minutes = state.minutes;
		let power = state.power;
		let open = state.open.clone();
		if minutes == 0 {
			if pressure > best {
				best = best.max(pressure);
				if memorize {
					best_states.clear();
					best_states.insert(state);
				}
				//println!("{state:?}");
				//println!("Found max {best} ({} states)", states.len());
			} else if memorize && pressure == best {
				best_states.insert(state);
			}
			continue;
		}
		let current = state.current;

		if pressure + max_power * minutes <= best {
			// No need to go on
			continue;
		}
		// Check the early cut
		// If an early cut is set and impossible to reach, the result we get
		// is necessarily *not* actually the best, but we still get there faster
		if let Some(cut) = early_cut {
			if pressure + max_power * minutes <= cut {
				// No need to bother
				continue;
			}
		}

		// Where can we go and open?
		for (target, cost) in &distances[&current] {
			// No need to go somewhere you already opened
			if open.contains(&target.as_str()) {
				continue;
			}
			// This will re-add "AA" as a state we can get to but it's always
			// either a waste of time or a necessary step to the right output
			// so...
			if *cost+1 > minutes {
				// no need
				continue;
			}
			// Go there and open
			let pressure = pressure + (cost + 1) * power;
			let mut new_open = open.clone();
			new_open.push(target);
			let new_state = MazeState::build(
				minutes-cost-1, pressure,
				target.clone(),
				power + maze[target].0, new_open
			);
			if !seen.contains(&new_state) {
				seen.insert(new_state.clone());
				states.push(new_state);
			}
		}

		// What if we just... stopped?
		let new_state = MazeState::build(0, pressure + minutes * power,
										 //format!("{}, and wait", state.orders),
										 current, power, open.clone());
		if !seen.contains(&new_state) {
			seen.insert(new_state.clone());
			states.push(new_state);
		}
	}

	(best, if memorize { Some(best_states) } else { None })
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
	// Djikstra to establish initial distances used for everything else
	for valve_one in &non_null_valves {
		for valve_two in &non_null_valves {
			// Ew
			if valve_one == valve_two { continue; }
			let valve_one = (*valve_one).to_string();
			let valve_two = (*valve_two).to_string();
			let dist = distance(&maze, &valve_one, &valve_two);
			distances.get_mut(&valve_one).expect("fetchable")
					.insert(valve_two.clone(), dist);
			//distances.get_mut(&valve_two).expect("fetchable")
					//.insert(valve_one.clone(), dist);
			assert_eq!(distance(&maze, &valve_one, &valve_two), distance(&maze, &valve_two, &valve_one));
		}
	}

	// So.. long time no see, itertools
	// No need to simulate both at once :)
	let nodes = non_null_valves.iter().map(std::string::ToString::to_string).collect::<HashSet<String>>();
	//println!("{} nodes", nodes.len());
	let mut total_best = 0;
	for size in (((nodes.len()/2)-1) ..= nodes.len()/2).rev() {
		//println!("Size = {size}");
		let combinations = nodes.iter().combinations(size);
		//println!("{} combinations", combinations.clone().count());
		for human_seen in combinations {
			let human_seen: Vec<&str> = human_seen.iter().map(|s| s.as_str()).collect();
			let elephant_seen = nodes.iter()
				.filter_map(|n| {
					let vq = n.as_str();
					if human_seen.contains(&vq) {
						None
					} else {
						Some(vq)
					}
				})
				.collect::<Vec<&str>>();
			// Run
			let human_best = compute(&maze, &distances, None, 26, human_seen, false).0;
			let elephant_best = compute(&maze, &distances, usize::checked_sub(total_best, human_best), 26, elephant_seen, false).0;
			let best = human_best + elephant_best;
			if best > total_best {
				//println!("New best: {best}");
				total_best = best;
			}
		}
	}
	total_best
}
