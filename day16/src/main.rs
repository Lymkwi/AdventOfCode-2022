//! This crates contains the code necessary to solve Advent of Code day 16,
//! all written in Rust.

extern crate common;
use common::read_data;
use day16::{solve_part_one, solve_part_two};

#[doc(hidden)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let data = read_data("input")?;
	println!("{}", solve_part_one(&data));
	println!("{}", solve_part_two(&data));
	Ok(())
}


#[cfg(test)]
mod test {
	use super::*;
	use common::test;

	test!(day16_01_example1, 1, 1651, "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II");
	test!(day16_01_example2, 1, 1701, "Valve SW has flow rate=0; tunnels lead to valves LX, LD\nValve VS has flow rate=0; tunnels lead to valves JO, OO\nValve OO has flow rate=10; tunnels lead to valves KK, HD, VS, KI\nValve DZ has flow rate=8; tunnels lead to valves KV, GX, WQ, BA, PK\nValve GX has flow rate=0; tunnels lead to valves AA, DZ\nValve IF has flow rate=0; tunnels lead to valves OI, DW\nValve BO has flow rate=0; tunnels lead to valves UJ, ZT\nValve KI has flow rate=0; tunnels lead to valves OO, KU\nValve JT has flow rate=3; tunnels lead to valves FC, AM, KV, XP, XZ\nValve TQ has flow rate=0; tunnels lead to valves AA, DW\nValve KK has flow rate=0; tunnels lead to valves QW, OO\nValve NR has flow rate=0; tunnels lead to valves UG, XM\nValve VO has flow rate=0; tunnels lead to valves YR, AA\nValve MS has flow rate=17; tunnels lead to valves LT, LX\nValve JO has flow rate=0; tunnels lead to valves YR, VS\nValve ZB has flow rate=0; tunnels lead to valves UJ, LT\nValve ZT has flow rate=0; tunnels lead to valves XM, BO\nValve YR has flow rate=9; tunnels lead to valves VO, FY, WB, JO\nValve QS has flow rate=0; tunnels lead to valves QW, FY\nValve UD has flow rate=0; tunnels lead to valves CA, JB\nValve AP has flow rate=0; tunnels lead to valves CA, DW\nValve KV has flow rate=0; tunnels lead to valves JT, DZ\nValve JH has flow rate=0; tunnels lead to valves IK, UJ\nValve LD has flow rate=15; tunnels lead to valves IK, SW\nValve XK has flow rate=0; tunnels lead to valves XZ, BH\nValve XM has flow rate=11; tunnels lead to valves XP, CJ, ZT, NR\nValve FY has flow rate=0; tunnels lead to valves YR, QS\nValve GI has flow rate=22; tunnel leads to valve TI\nValve JB has flow rate=14; tunnels lead to valves WB, UD, WQ, HD\nValve DW has flow rate=6; tunnels lead to valves AP, TQ, NQ, IF, PK\nValve UJ has flow rate=13; tunnels lead to valves JH, ZB, BO\nValve KU has flow rate=0; tunnels lead to valves CA, KI\nValve WQ has flow rate=0; tunnels lead to valves JB, DZ\nValve BA has flow rate=0; tunnels lead to valves BH, DZ\nValve AA has flow rate=0; tunnels lead to valves YX, TQ, VO, GX, QP\nValve TI has flow rate=0; tunnels lead to valves GI, UG\nValve FC has flow rate=0; tunnels lead to valves QP, JT\nValve CA has flow rate=18; tunnels lead to valves KU, UD, AP\nValve QW has flow rate=25; tunnels lead to valves QS, KK\nValve XZ has flow rate=0; tunnels lead to valves JT, XK\nValve YX has flow rate=0; tunnels lead to valves AA, CJ\nValve OI has flow rate=0; tunnels lead to valves IF, BH\nValve NQ has flow rate=0; tunnels lead to valves AM, DW\nValve QP has flow rate=0; tunnels lead to valves AA, FC\nValve AM has flow rate=0; tunnels lead to valves NQ, JT\nValve XP has flow rate=0; tunnels lead to valves XM, JT\nValve BH has flow rate=12; tunnels lead to valves BA, XK, OI\nValve HD has flow rate=0; tunnels lead to valves OO, JB\nValve LT has flow rate=0; tunnels lead to valves MS, ZB\nValve LX has flow rate=0; tunnels lead to valves MS, SW\nValve CJ has flow rate=0; tunnels lead to valves XM, YX\nValve PK has flow rate=0; tunnels lead to valves DW, DZ\nValve IK has flow rate=0; tunnels lead to valves LD, JH\nValve WB has flow rate=0; tunnels lead to valves YR, JB\nValve UG has flow rate=21; tunnels lead to valves TI, NR");
	test!(day16_02_example1, 2, 1707, "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II");
}
