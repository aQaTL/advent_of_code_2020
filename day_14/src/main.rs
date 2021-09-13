#![feature(asm)]

use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_14/input.txt")?;
	let input = parse_input(&input)?;

	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Op {
	MaskSet {
		and_mask: u64,
		or_mask: u64,
		floating_mask: u64,
	},
	MemSet {
		addr: u64,
		value: u64,
	},
}

fn parse_input(input: &str) -> Result<Vec<Op>> {
	let re = regex::Regex::new(
		r"^(mask = (?P<mask>\w+))|(mem\[(?P<mem_addr>\d+)\] = (?P<mem_value>\d+))$",
	)?;
	let mut ops = Vec::<Op>::new();
	for line in input.lines() {
		let captures = re
			.captures(line)
			.ok_or_else(|| anyhow!("failed to match"))?;
		if let Some(mask_cap) = captures.name("mask") {
			let mut and_mask = !0;
			let mut or_mask = 0;
			let mut floating_mask = 0;

			for (idx, b) in mask_cap
				.as_str()
				.as_bytes()
				.iter()
				.copied()
				.rev()
				.enumerate()
			{
				if b == b'0' {
					and_mask ^= 1 << idx;
				} else if b == b'1' {
					or_mask |= 1 << idx;
				} else if b == b'X' {
					floating_mask |= 1 << idx;
				}
			}
			ops.push(Op::MaskSet {
				and_mask,
				or_mask,
				floating_mask,
			});

			continue;
		}
		if let (Some(mem_addr), Some(mem_value)) =
			(captures.name("mem_addr"), captures.name("mem_value"))
		{
			let addr = mem_addr.as_str().parse()?;
			let value = mem_value.as_str().parse()?;
			ops.push(Op::MemSet { addr, value });

			continue;
		}

		bail!("no matches found in line {}", line);
	}
	Ok(ops)
}

fn part_1(input: &[Op]) -> u64 {
	let mut mem = HashMap::<u64, u64>::new();
	let (mut and_mask, mut or_mask) = (!0, 0);

	for op in input.iter().cloned() {
		match op {
			Op::MaskSet {
				and_mask: new_and_mask,
				or_mask: new_or_mask,
				..
			} => {
				and_mask = new_and_mask;
				or_mask = new_or_mask;
			}
			Op::MemSet { addr, value } => {
				let mem_cell = mem.entry(addr).or_default();
				*mem_cell = value & and_mask | or_mask;
			}
		}
	}

	mem.values().sum::<u64>()
}

fn part_2(input: &[Op]) -> u64 {
	let mut mem = HashMap::<u64, u64>::new();
	let (mut or_mask, mut floating_mask) = (0, 0);

	for op in input.iter().cloned() {
		match op {
			Op::MaskSet {
				or_mask: new_or_mask,
				floating_mask: new_floating_mask,
				..
			} => {
				or_mask = new_or_mask;
				floating_mask = new_floating_mask;
			}
			Op::MemSet { addr, value } => {
				let bits_count = floating_mask.count_ones();
				for mini_mask in 0..(2_u64.pow(bits_count)) {
					let mut curr_bit_idx = bits_count - 1;

					let mut floating_mask = floating_mask;
					let (mut floating_and_mask, mut floating_or_mask) = (!0, 0);

					loop {
						let msb = most_significant_bit(floating_mask);

						if mini_mask & (1 << curr_bit_idx) == 0 {
							floating_and_mask ^= msb;
						} else {
							floating_or_mask |= msb;
						}

						floating_mask &= !msb;
						if floating_mask == 0 {
							break;
						}
						curr_bit_idx -= 1;
					}

					let addr = addr & floating_and_mask | floating_or_mask | or_mask;
					let mem_cell = mem.entry(addr).or_default();
					*mem_cell = value;
				}
			}
		}
	}

	mem.values().sum::<u64>()
}

fn most_significant_bit_idx(n: u64) -> u64 {
	let msb_idx: u64;
	unsafe {
		asm!("bsr {0}, {1}", out(reg) msb_idx, in(reg) n);
	}
	msb_idx
}

fn most_significant_bit(n: u64) -> u64 {
	if n == 0 {
		return 0;
	}
	1 << most_significant_bit_idx(n)
}

#[cfg(test)]
mod tests {
	#[test]
	fn bitmask() {
		let value: u64 = 0b_1100101;
		let and_mask: u64 = 0b_1111101;
		let or_mask: u64 = 0b_1000000;
		let result = value & and_mask | or_mask;
		println!("{:b}", result);
		assert_eq!(result, 0b1100101);
	}

	#[test]
	fn msb() {
		let x: u64 = 0b1010101011;
		let expected: u64 = 0b1000000000;
		assert_eq!(super::most_significant_bit(x), expected);

		let x: u64 = 0b101;
		let expected: u64 = 4;
		assert_eq!(super::most_significant_bit(x), expected);

		let x: u64 = 0b0;
		let expected: u64 = 0;
		assert_eq!(super::most_significant_bit(x), expected);
	}

	#[test]
	fn msb_idx() {
		let x = 0;
		let expected = 0;
		assert_eq!(super::most_significant_bit_idx(x), expected);
		let x = 1;
		let expected = 0;
		assert_eq!(super::most_significant_bit_idx(x), expected);
		let x = 0b10;
		let expected = 1;
		assert_eq!(super::most_significant_bit_idx(x), expected);
	}

	const EX1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

	#[test]
	fn ex1() {
		let input = super::parse_input(EX1).unwrap();
		let result = super::part_1(&input);
		assert_eq!(result, 165);
	}

	const EX2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

	#[test]
	fn ex2() {
		let input = super::parse_input(EX2).unwrap();
		let result = super::part_2(&input);
		assert_eq!(result, 208)
	}
}
