use criterion::{criterion_group, criterion_main, Criterion};
use common::read_data;

macro_rules! day_tests {
	($daynum:literal, $crit:expr, $daycrate:ident) => {
		let data = read_data(&format!("day{:02}/input", $daynum)).unwrap();
		$crit.bench_function(&format!("day{:02}-1", $daynum),
			|b| b.iter(|| $daycrate::solve_part_one(&data)));
		$crit.bench_function(&format!("day{:02}-2", $daynum),
			|b| b.iter(|| $daycrate::solve_part_two(&data)));
	}
}

pub fn criterion_benchmark(c: &mut Criterion) {
	day_tests!(1, c, day01);
	day_tests!(2, c, day02);
	day_tests!(3, c, day03);
	day_tests!(4, c, day04);
	day_tests!(5, c, day05);
	day_tests!(6, c, day06);
	day_tests!(7, c, day07);
	day_tests!(8, c, day08);
	day_tests!(9, c, day09);
	day_tests!(10, c, day10);
	day_tests!(11, c, day11);
	day_tests!(12, c, day12);
	day_tests!(13, c, day13);
	day_tests!(14, c, day14);
	day_tests!(15, c, day15);
	day_tests!(16, c, day16);
	day_tests!(17, c, day17);
	day_tests!(18, c, day18);
	day_tests!(19, c, day19);
	day_tests!(20, c, day20);
	day_tests!(21, c, day21);
	day_tests!(22, c, day22);
	day_tests!(23, c, day23);
	day_tests!(24, c, day24);
	day_tests!(25, c, day25);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
