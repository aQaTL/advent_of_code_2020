use itertools::Itertools;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input: Vec<u64> = std::fs::read_to_string("day_10/input.txt")?
		.lines()
		.map(str::parse)
		.try_collect()?;
	println!("Part 1: {}", part_1(input.clone()));
	println!("Part 2: {}", part_2(input));
	Ok(())
}

fn part_1(mut input: Vec<u64>) -> u64 {
	input.sort();

	let (ones, threes): (Vec<u64>, Vec<u64>) = (0..=0)
		.chain(input.into_iter())
		.tuple_windows()
		.map(|(a, b)| b - a)
		.partition(|diff| *diff == 1);

	ones.iter().count() as u64 * (threes.iter().count() as u64 + 1)
}

fn part_2(mut input: Vec<u64>) -> u64 {
	input.sort();
	input.insert(0, 0);
	input.push(*input.last().unwrap() + 3);

	calc(&mut HashMap::new(), &input, 0)
}

fn calc(mem: &mut HashMap<usize, u64>, input: &[u64], idx: usize) -> u64 {
	if idx <= input.len() - 4 {
		let a = if input[idx + 1] - input[idx] <= 3 {
			get_or_calc(mem, input, idx + 1)
		} else {
			0
		};
		let b = if input[idx + 2] - input[idx] <= 3 {
			get_or_calc(mem, input, idx + 2)
		} else {
			0
		};
		let c = if input[idx + 3] - input[idx] <= 3 {
			get_or_calc(mem, input, idx + 3)
		} else {
			0
		};
		a + b + c
	} else if idx == input.len() - 3 {
		let a = if input[idx + 1] - input[idx] <= 3 {
			get_or_calc(mem, input, idx + 1)
		} else {
			0
		};
		let b = if input[idx + 2] - input[idx] <= 3 {
			get_or_calc(mem, input, idx + 2)
		} else {
			0
		};
		a + b
	} else if idx == input.len() - 2 {
		1
	} else {
		0
	}
}

fn get_or_calc(mem: &mut HashMap<usize, u64>, input: &[u64], idx: usize) -> u64 {
	match mem.get(&idx).copied() {
		Some(v) => v,
		None => {
			let x = calc(mem, input, idx);
			mem.insert(idx, x);
			x
		}
	}
}
