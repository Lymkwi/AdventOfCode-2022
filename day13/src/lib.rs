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
	cmp::{
		Eq,
		Ordering,
		PartialEq,
		PartialOrd,
	},
	collections::VecDeque,
	iter::zip,
};

/// Solve Advent of Code day 13 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 13.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	data.split("\n\n")
		.enumerate()
		.filter_map(|(idx, st)| {
			let packets: Vec<Packet> = st.split('\n')
				.map(|x| x.parse::<Packet>().unwrap()).collect::<Vec<_>>();
			if packets[0] < packets[1] {
				Some(idx+1)
			} else {
				None
			}
		})
	.sum()
}

#[derive(Debug, Clone)]
enum PacketItem {
	Int(usize),
	Packet(Packet)
}

impl PartialEq for PacketItem {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			// If both are numbers, compare
			(Self::Int(a), Self::Int(b)) => a == b,
			// If both are packets, compare the packets
			(Self::Packet(a), Self::Packet(b)) => a == b,
			// If only one is an Int, create a packet with only it and compare
			(Self::Packet(a), Self::Int(b)) => {
				let pb = Packet::from_int(*b);
				*a == pb
			},
			(Self::Int(a), Self::Packet(b)) => {
				let pa = Packet::from_int(*a);
				pa == *b
			}
		}
	}
}

impl Eq for PacketItem {}

impl PartialOrd for PacketItem {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
			(Self::Packet(a), Self::Packet(b)) => a.partial_cmp(b),
			(Self::Packet(a), Self::Int(b)) => {
				let pb = Packet::from_int(*b);
				a.partial_cmp(&pb)
			},
			(Self::Int(a), Self::Packet(b)) => {
				let pa = Packet::from_int(*a);
				pa.partial_cmp(b)
			}
		}
	}
}

#[derive(Debug, Clone)]
struct Packet {
	data: Vec<PacketItem>
}

impl Packet {
	fn from_int(val: usize) -> Self {
		Self { data: vec![PacketItem::Int(val)] }
	}

	fn from_packet(pa: Self) -> Self {
		Self { data: vec![PacketItem::Packet(pa)] }
	}
}

impl PartialEq for Packet {
	fn eq(&self, other: &Self) -> bool {
		// Compare lengths
		self.data.len() == other.data.len()
			&& zip(&self.data, &other.data)
				.all(|(a, b)| a == b)
	}
}

impl Eq for Packet {}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// We're not actually going to reuse the PartialEq/Eq implementations,
		// because using them individually is much less efficient
		let mut stack: VecDeque<(&PacketItem, &PacketItem)> = zip(&self.data, &other.data).collect();
		while !stack.is_empty() {
			let (left, right) = stack.pop_front().unwrap();
			match (left, right) {
				(PacketItem::Int(a), PacketItem::Int(b)) => {
					if a == b {
						continue;
					}
					return a.partial_cmp(b);
				},
				(PacketItem::Packet(a), PacketItem::Packet(b)) => {
					// Input shouldn't be too bad...
					if a == b {
						continue;
					}
					return a.partial_cmp(b);
				},
				(PacketItem::Packet(pa), PacketItem::Int(b)) => {
					let pb = Self::from_int(*b);
					if *pa == pb {
						continue;
					}
					return pa.partial_cmp(&pb);
				},
				(PacketItem::Int(a), PacketItem::Packet(pb)) => {
					let pa = Self::from_int(*a);
					if pa == *pb {
						continue;
					}
					return pa.partial_cmp(pb);
				}
			}
		}

		// If we get here, either side has run out, or we're equal
		// So the result is just a size comparison
		self.data.len().partial_cmp(&other.data.len())
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl std::str::FromStr for Packet {
	type Err = ();
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		// Packet should have `[` and `]`
		let mut st_it = st.chars();
		st_it.next();
		st_it.next_back();
		let mut data: Vec<PacketItem> = Vec::new();
		let mut digit_buffer = String::new();

		// We'll now eat st_it
		while let Some(c) = st_it.next() {
			if c.is_ascii_digit() {
				// append to current digit_buffer
				digit_buffer.push(c);
			} else if c == ',' && !digit_buffer.is_empty() {
				// Terminate the digit buffer
				let number = digit_buffer.parse::<usize>().unwrap();
				data.push(PacketItem::Int(number));
				digit_buffer.clear();
			} else if c == '[' {
				// Thal help us
				// Collect the entire underlying list in a string, parse that,
				// and then repeat
				let mut packet_buffer: String = "[".into();
				let mut bracket_count = 1;
				while bracket_count > 0 {
					let k = st_it.next().expect("No next");
					if k == '[' {
						bracket_count += 1;
					} else if k == ']' {
						bracket_count -= 1;
					}
					packet_buffer.push(k);
				}
				let pa = packet_buffer.parse::<Self>().unwrap();
				data.push(PacketItem::Packet(pa));
			}
		}

		if !digit_buffer.is_empty() {
			let number = digit_buffer.parse::<usize>().unwrap();
			data.push(PacketItem::Int(number));
		}

		Ok(Self { data })
	}
}

/// Solve Advent of Code day 13 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 13.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
	let mut packets = data.split('\n')
		.filter_map(|x| {
			if x.is_empty() {
				None
			} else { Some(x.parse::<Packet>().unwrap()) }
		}).collect::<Vec<Packet>>();
	let segment_one = Packet::from_packet(Packet::from_int(2));
	let segment_two = Packet::from_packet(Packet::from_int(6));

	packets.push(segment_one.clone());
	packets.push(segment_two.clone());
	packets.sort_unstable();
	let mut packets_it = packets.iter();
	(packets_it.clone().position(|s| *s == segment_one).expect("One") + 1) *
		(packets_it.position(|s| *s == segment_two).expect("Two") + 1)
}

// vim: set tw=80:
