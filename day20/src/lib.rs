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

/// Solve Advent of Code day 20 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `isize`, the result for part one of advent of code
/// day 20.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> isize {
	let numbers = data.trim().split('\n')
		.map(|s| s.parse::<isize>().expect("convertible"))
		.enumerate()
		.map(|(u, k)| (k, u))
		.collect::<Vec<Clef>>();

	let mut ribbon = Ribbon::build(&numbers);

	data.trim().split('\n')
		.map(|s| s.parse::<isize>().expect("convertible"))
		.enumerate()
		.for_each(|(u, k)| ribbon.mix((k, u)));

	//println!("{ribbon}");
	//println!("Len={}", ribbon.len());
	let a = ribbon[1000];
	let b = ribbon[2000];
	let c = ribbon[3000];

	//println!("a={a}, b={b}, c={c}");

	a + b + c
}

// Since it turns out that not all numbers are unique, we will use the pair of
// (value, position in original list) as being a unique key
type Clef = (isize, usize);

struct Ribbon {
	data: HashMap<Clef, (Clef, Clef)>,
	len: usize,
	head: Clef,
}

impl Ribbon {
	const fn len(&self) -> usize {
		self.len
	}
}

impl Ribbon {
	fn mix(&mut self, key: Clef) {
		let (num, _) = key;
		if num == 0 { return; }

		let size_signed: isize = (self.len - 1).try_into()
			.expect("convertible");
		let mut absolute_num: isize = isize::abs(num);
		absolute_num %= size_signed;
		if num < 0 {
			// Going backwards, so change it
			absolute_num = size_signed - absolute_num;
		}
		let mut other = key;
		for _ in 0..absolute_num {
			// Move forward by `absolute_num`
			other = self.data[&other].1;
		}

		// Now change this
		let other_links = self.data[&other];
		let my_links = self.data[&key];

		self.data.insert(other, (other_links.0, key));
		self.data.insert(key, (other, other_links.1));
		// Stitch back the thing that followed "other"
		let after_other = self.data[&other_links.1];
		self.data.insert(other_links.1, (key, after_other.1));
		// Stitch back where we removed "key"
		let links_after = self.data[&my_links.1];
		let links_before = self.data[&my_links.0];
		self.data.insert(my_links.1, (my_links.0, links_after.1));
		self.data.insert(my_links.0, (links_before.0, my_links.1));
	} 

	fn build(numbers: &[Clef]) -> Self {
		let mut data: HashMap<Clef, (Clef, Clef)> = HashMap::new();

		numbers[..]
			.windows(3)
			.for_each(|nums| {
				let one = nums[0];
				let two = nums[1];
				let three = nums[2];

				data.insert(two, (one, three));
			});
		let len: usize = numbers.len();
		let head = (0, numbers.iter().position(|s| s.0 == 0).expect("one zero"));

		// Tie up the loose ends
		let list_head = numbers[0];
		let list_tail = *numbers.last().expect("At least one number");
		let tail_m_one = numbers[numbers.len()-2];
		let head_p_one = numbers[1];

		data.insert(list_head, (list_tail, head_p_one));
		data.insert(list_tail, (tail_m_one, list_head));

		Self { data, len, head }
	}
}

impl std::fmt::Display for Ribbon {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// ok
		write!(fmt, "{:?}", (0..self.len())
			   .map(|s| self[s].to_string()).collect::<Vec<String>>())
	}
}

impl std::ops::Index<usize> for Ribbon {
	type Output = isize;
	fn index(&self, idx: usize) -> &Self::Output {
		// First, don't run around needlessly
		let rounded_idx = idx % self.data.len();
		//// What way should we go?
		if rounded_idx < self.data.len() / 2 {
			// Go forward
			let mut answer: &Clef = &self.head;
			for _ in 0..rounded_idx {
				answer = &self.data[answer].1;
			}
			&answer.0
		} else {
			// Go backwards
			let rounded_idx = self.data.len() - rounded_idx;
			let mut answer: &Clef = &self.head;
			for _ in 0..rounded_idx {
				answer = &self.data[answer].0;
			}
			&answer.0
		}
	} 
}

/// Solve Advent of Code day 20 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `isize`, the result for part
/// two of advent of code day 20.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> isize {
	let decryption_key = 811_589_153;
	let numbers = data.trim().split('\n')
		.map(|s| s.parse::<isize>().expect("convertible") * decryption_key)
		.enumerate()
		.map(|(u, k)| (k, u))
		.collect::<Vec<Clef>>();

	let mut ribbon = Ribbon::build(&numbers);

	for _ in 0..10 {
		numbers.iter().for_each(|&key| ribbon.mix(key));
	}

	//println!("{ribbon}");
	//println!("Len={}", ribbon.len());
	let a = ribbon[1000];
	let b = ribbon[2000];
	let c = ribbon[3000];

	//println!("a={a}, b={b}, c={c}");

	a + b + c
}

// vim: set tw=80:
