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
				match mem.get_mut(&0) {
					Some((_last_turn, ref mut opt @ None)) => {
						*opt = Some(turn);
					}
					Some((last_turn, Some(last_turn_2))) => {
						*last_turn = *last_turn_2;
						*last_turn_2 = turn;
					}
					None => {
						mem.insert(0, (turn, None));
					}
				}
				last_num = 0;
			}
			Some((last_turn, Some(last_turn_2))) => {
				last_num = *last_turn_2 - *last_turn;
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
			None => {
				bail!("invalid state");
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
		assert_eq!(super::part_1(EX1).unwrap(), EX1_SOLUTION);
	}
	#[test]
	fn ex2() {
		assert!(matches!(super::part_1(EX2), Ok(EX2_SOLUTION)));
	}
	#[test]
	fn ex3() {
		assert!(matches!(super::part_1(EX3), Ok(EX3_SOLUTION)));
	}
	#[test]
	fn ex4() {
		assert!(matches!(super::part_1(EX4), Ok(EX4_SOLUTION)));
	}
	#[test]
	fn ex5() {
		assert!(matches!(super::part_1(EX5), Ok(EX5_SOLUTION)));
	}
	#[test]
	fn ex6() {
		assert!(matches!(super::part_1(EX6), Ok(EX6_SOLUTION)));
	}
	#[test]
	fn ex7() {
		assert!(matches!(super::part_1(EX7), Ok(EX7_SOLUTION)));
	}
}
