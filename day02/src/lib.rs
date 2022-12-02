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

// All of this code is globally very over-engineered, but I want to flex my Rust
// muscles more than I want to make efficient code at the moment

/// Solve Advent of Code day 02 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today
///
///  The data has the shape of lines of `L1 L2` where `L1` is either `X`, `Y` or
///  `Z`, and `L2` is one of `A`, `B` or `C`. Those are all, in order, rock ✊,
///  paper ✋ and scissors ✌.
///  `L1` is the hand your opponent will play, and `L2` is yours.
///
/// # Return value
///
/// In first part, we only compute our score for the rounds, with the following
/// method of computation:
///  - For each round, you obtain two counts of points, from the outcome of the
///  round and the hand you played
///  - In terms of outcome, you get 0 when you lose, 3 for a draw, and 6 for a
///  win
///  - In terms of hand, you get 1 for a rock, 2 for a hand, and 3 for scissors
///
/// We return the total score for the paper guide given to us.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
	data.trim().split('\n')
		.map(|line| {
			let its = line.chars().collect::<Vec<char>>();
			let theirs = Hand::new(*its.first().unwrap());
			let mine = Hand::new(*its.get(2).unwrap());
			let outcome = match mine.partial_cmp(&theirs).unwrap() {
				std::cmp::Ordering::Less => 0,
				std::cmp::Ordering::Equal => 3,
				std::cmp::Ordering::Greater => 6
			};
			let hand_score = match mine {
				Hand::Rock => 1,
				Hand::Paper => 2,
				Hand::Scissors => 3
			};
			outcome + hand_score
		})
	.sum()
}

/// One of the three hands that can be played
#[derive(PartialEq, Eq)]
enum Hand {
	/// The Rock hand ✊
	Rock,
	/// The Paper hand ✋
	Paper,
	/// The Scissors hand ✌
	Scissors
}

impl Hand {
	/// Build a new `Hand` based on the character given
	fn new(code: char) -> Self {
		match code {
			'X' | 'A' => Self::Rock,
			'Y' | 'B' => Self::Paper,
			'Z' | 'C' => Self::Scissors,
			_ => unreachable!()
		}
	}

	/// Returns what hand would win over ours
	const fn winner_over(&self) -> Self {
		match self {
			Self::Rock => Self::Paper,
			Self::Paper => Self::Scissors,
			Self::Scissors => Self::Rock
		}
	}

	/// Returns what hand would lose over ours
	const fn loser_over(&self) -> Self {
		match self {
			Self::Rock => Self::Scissors,
			Self::Paper => Self::Rock,
			Self::Scissors => Self::Paper
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(match self {
			Self::Rock => {
				match other {
					Self::Rock => std::cmp::Ordering::Equal,
					Self::Paper => std::cmp::Ordering::Less,
					Self::Scissors => std::cmp::Ordering::Greater
				}
			},
			Self::Paper => {
				match other {
					Self::Rock => std::cmp::Ordering::Greater,
					Self::Paper => std::cmp::Ordering::Equal,
					Self::Scissors => std::cmp::Ordering::Less
				}
			},
			Self::Scissors => {
				match other {
					Self::Rock => std::cmp::Ordering::Less,
					Self::Paper => std::cmp::Ordering::Greater,
					Self::Scissors => std::cmp::Ordering::Equal
				}
			}
		})
	}
}

/// Solve Advent of Code day 02 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds today's input
///
///  In part 2, the input is understood to still mean "the hands to play",
///  except this time the second column tells you whether you are supposed to
///  lose (`X`), draw (`Y`) or win (`Z`). From there, we need to reverse
///  engineer what hand we will play, and use that one.
///
/// # Return value
///
/// The output is still the score computed from the hands we play.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
	data.trim().split('\n')
		.map(|line| {
			let its = line.chars().collect::<Vec<char>>();
			let theirs = Hand::new(*its.first().unwrap());
			let outcome = match its.get(2).unwrap() {
				'X' => 0,
				'Y' => 3,
				'Z' => 6,
				_ => unreachable!()
			};
			// Now reverse engineer our hand
			let mine = match outcome {
				0 => theirs.loser_over(),
				3 => theirs,
				6 => theirs.winner_over(),
				_ => unreachable!()
			};
			let hand_score = match mine {
				Hand::Rock => 1,
				Hand::Paper => 2,
				Hand::Scissors => 3
			};
			outcome + hand_score
		})
	.sum()
}

