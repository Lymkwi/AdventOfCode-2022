// Clippy lints!
//
// https://vulpinecitrus.info/blog/clippy-obey-the-paperclip
// :)
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

#![allow(unused_variables)]

fn main() {
	println!("Use `cargo bench` or `cargo test`.");
}

#[cfg(test)]
mod test {
	macro_rules! result_tests {
		($daycrate:ident, $func_one:ident, $func_two:ident, $day:literal, $res1:literal, $res2:literal) => {
			#[test]
			fn $func_one() {
				assert_eq!($res1,
					$daycrate::solve_part_one(
						&read_data(&format!("day{:02}/input", $day)).unwrap()))
			}

			#[test]
			fn $func_two() {
				assert_eq!($res2,
				   $daycrate::solve_part_two(
						&read_data(&format!("day{:02}/input", $day)).unwrap()))
			}
		}
	}

	use common::read_data;

	result_tests!(day01, day01_one, day01_two, 01, 72511, 212117);
	result_tests!(day02, day02_one, day02_two, 02, 0, 0);
	result_tests!(day03, day03_one, day03_two, 03, 0, 0);
	result_tests!(day04, day04_one, day04_two, 04, 0, 0);
	result_tests!(day05, day05_one, day05_two, 05, 0, 0);
	result_tests!(day06, day06_one, day06_two, 06, 0, 0);
	result_tests!(day07, day07_one, day07_two, 07, 0, 0);
	result_tests!(day08, day08_one, day08_two, 08, 0, 0);
	result_tests!(day09, day09_one, day09_two, 09, 0, 0);
	result_tests!(day10, day10_one, day10_two, 10, 0, 0);
	result_tests!(day11, day11_one, day11_two, 11, 0, 0);
	result_tests!(day12, day12_one, day12_two, 12, 0, 0);
	result_tests!(day13, day13_one, day13_two, 13, 0, 0);
	result_tests!(day14, day14_one, day14_two, 14, 0, 0);
	result_tests!(day15, day15_one, day15_two, 15, 0, 0);
	result_tests!(day16, day16_one, day16_two, 16, 0, 0);
	result_tests!(day17, day17_one, day17_two, 17, 0, 0);
	result_tests!(day18, day18_one, day18_two, 18, 0, 0);
	result_tests!(day19, day19_one, day19_two, 19, 0, 0);
	result_tests!(day20, day20_one, day20_two, 20, 0, 0);
	result_tests!(day21, day21_one, day21_two, 21, 0, 0);
	result_tests!(day22, day22_one, day22_two, 22, 0, 0);
	result_tests!(day23, day23_one, day23_two, 23, 0, 0);
	result_tests!(day24, day24_one, day24_two, 24, 0, 0);
	result_tests!(day25, day25_one, day25_two, 25, 0, 0);
}
