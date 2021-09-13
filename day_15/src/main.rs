use anyhow::{bail, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_15/input.txt")?;
	println!("Part 1: {}", solve(&input, 2020)?);
	println!("Part 2: {}", solve(&input, 30000000)?);
	Ok(())
}

fn solve(input: &str, cutoff: u64) -> Result<u64> {
	let input: Vec<u64> = input
		.trim()
		.split(",")
		.filter_map(|num| num.parse::<u64>().ok())
		.collect();

	let mut mem = HashMap::new();

	for (idx, n) in input.iter().copied().enumerate() {
		mem.insert(n, (idx as u64 + 1, None));
	}

	let mut last_num = *input.last().unwrap();

	for turn in (input.len() as u64 + 1)..=cutoff {
		match mem.get_mut(&last_num) {
			Some((_last_turn, None)) => {
				last_num = 0;
			}
			Some((last_turn, Some(last_turn_2))) => {
				last_num = *last_turn_2 - *last_turn;
			}
			None => {
				bail!("invalid state");
			}
		}
		match mem.get_mut(&last_num) {
			Some((_last_turn, ref mut opt @ None)) => {
				*opt = Some(turn);
			}
			Some((last_turn, Some(last_turn_2))) => {
				*last_turn = *last_turn_2;
				*last_turn_2 = turn;
			}
			None => {
				mem.insert(last_num, (turn, None));
			}
		}
	}

	Ok(last_num)
}

#[cfg(test)]
mod tests {
	const EX1: &str = "0,3,6";
	const EX1_SOLUTION: u64 = 436;
	const EX2: &str = "1,3,2";
	const EX2_SOLUTION: u64 = 1;
	const EX3: &str = "2,1,3";
	const EX3_SOLUTION: u64 = 10;
	const EX4: &str = "1,2,3";
	const EX4_SOLUTION: u64 = 27;
	const EX5: &str = "2,3,1";
	const EX5_SOLUTION: u64 = 78;
	const EX6: &str = "3,2,1";
	const EX6_SOLUTION: u64 = 438;
	const EX7: &str = "3,1,2";
	const EX7_SOLUTION: u64 = 1836;

	#[test]
	fn ex1() {
		assert_eq!(super::solve(EX1, 2020).unwrap(), EX1_SOLUTION);
	}

	#[test]
	fn ex2() {
		assert_eq!(super::solve(EX2, 2020).unwrap(), EX2_SOLUTION);
	}

	#[test]
	fn ex3() {
		assert_eq!(super::solve(EX3, 2020).unwrap(), EX3_SOLUTION);
	}

	#[test]
	fn ex4() {
		assert_eq!(super::solve(EX4, 2020).unwrap(), EX4_SOLUTION);
	}

	#[test]
	fn ex5() {
		assert_eq!(super::solve(EX5, 2020).unwrap(), EX5_SOLUTION);
	}

	#[test]
	fn ex6() {
		assert_eq!(super::solve(EX6, 2020).unwrap(), EX6_SOLUTION);
	}

	#[test]
	fn ex7() {
		assert_eq!(super::solve(EX7, 2020).unwrap(), EX7_SOLUTION);
	}

	const EX1_P2: &str = "0,3,6";
	const EX1_P2_SOLUTION: u64 = 175594;
	const EX2_P2: &str = "1,3,2";
	const EX2_P2_SOLUTION: u64 = 2578;
	const EX3_P2: &str = "2,1,3";
	const EX3_P2_SOLUTION: u64 = 3544142;
	const EX4_P2: &str = "1,2,3";
	const EX4_P2_SOLUTION: u64 = 261214;
	const EX5_P2: &str = "2,3,1";
	const EX5_P2_SOLUTION: u64 = 6895259;
	const EX6_P2: &str = "3,2,1";
	const EX6_P2_SOLUTION: u64 = 18;
	const EX7_P2: &str = "3,1,2";
	const EX7_P2_SOLUTION: u64 = 362;

	#[test]
	fn ex1_p2() {
		assert_eq!(super::solve(EX1_P2, 30000000).unwrap(), EX1_P2_SOLUTION);
	}

	#[test]
	fn ex2_p2() {
		assert_eq!(super::solve(EX2_P2, 30000000).unwrap(), EX2_P2_SOLUTION);
	}

	#[test]
	fn ex3_p2() {
		assert_eq!(super::solve(EX3_P2, 30000000).unwrap(), EX3_P2_SOLUTION);
	}
	#[test]
	fn ex4_p2() {
		assert_eq!(super::solve(EX4_P2, 30000000).unwrap(), EX4_P2_SOLUTION);
	}
	#[test]
	fn ex5_p2() {
		assert_eq!(super::solve(EX5_P2, 30000000).unwrap(), EX5_P2_SOLUTION);
	}
	#[test]
	fn ex6_p2() {
		assert_eq!(super::solve(EX6_P2, 30000000).unwrap(), EX6_P2_SOLUTION);
	}
	#[test]
	fn ex7_p2() {
		assert_eq!(super::solve(EX7_P2, 30000000).unwrap(), EX7_P2_SOLUTION);
	}
}
