use itertools::Itertools;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
	let input: Vec<Op> = std::fs::read_to_string("day_8/input.txt")?
		.lines()
		.map(str::parse)
		.try_collect()?;
	part_1(input.clone());
	part_2(input);
	Ok(())
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Op {
	Nop(i64),
	Acc(i64),
	Jmp(i64),
}

impl FromStr for Op {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (op, val) = s.split(" ").collect_tuple().unwrap_or_default();
		let val = val.parse::<i64>()?;
		match op {
			"nop" => Ok(Op::Nop(val)),
			"acc" => Ok(Op::Acc(val)),
			"jmp" => Ok(Op::Jmp(val)),
			op => Err(anyhow::anyhow!("unknown op {}", op)),
		}
	}
}

fn part_1(input: Vec<Op>) {
	let mut pc: i64 = 0;
	let mut global_acc: i64 = 0;

	let mut history: Vec<(Vec<Op>, i64)> = Vec::new();

	loop {
		for (x, _) in history.iter_mut() {
			x.push(input[pc as usize]);
		}
		history.push((vec![input[pc as usize]], global_acc));
		match input[pc as usize] {
			Op::Nop(_) => pc += 1,
			Op::Jmp(val) => {
				pc += val;
				let c = history
					.iter()
					.find(|(x, _)| x[0..(x.len() / 2)] == x[(x.len() / 2..x.len())]);
				if let Some((c, _)) = c {
					let ans = history
						.iter()
						.find(|(x, _)| {
							if x.len() >= c.len() {
								x[(x.len() - c.len())..(x.len())] == c[..]
							} else {
								false
							}
						})
						.unwrap();
					println!("Part 1: {}", run(input, ans.0.len() - (c.len() / 2)).0);
					break;
				}
			}
			Op::Acc(val) => {
				global_acc += val;
				pc += 1;
			}
		}
	}
}

fn part_2(input: Vec<Op>) {
	let mut last_changed_idx = 0;

	loop {
		let mut input_2 = input.clone();
		for op in input_2.iter_mut().skip(last_changed_idx) {
			last_changed_idx += 1;
			match op {
				Op::Nop(val) => *op = Op::Jmp(*val),
				Op::Jmp(val) => *op = Op::Nop(*val),
				_ => continue,
			}
			break;
		}
		let ans = run(input_2, 1_000_000);
		if ans.1 as usize == input.len() {
			println!("Part 2: {}", ans.0);
			break;
		}
	}
}

fn run(input: Vec<Op>, max: usize) -> (i64, i64) {
	let mut pc: i64 = 0;
	let mut global_acc: i64 = 0;

	for _ in 0..max {
		if pc as usize == input.len() {
			break;
		}
		match input[pc as usize] {
			Op::Nop(_) => pc += 1,
			Op::Jmp(val) => pc += val,
			Op::Acc(val) => {
				global_acc += val;
				pc += 1;
			}
		}
	}
	(global_acc, pc)
}
