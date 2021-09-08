use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_13/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "939
7,13,x,x,59,x,31,19
";

	#[test]
	fn ex1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 295);
	}
	#[test]
	fn ex1_p2() {
		assert_eq!(super::part_2(EX_1).unwrap(), 1068781);
	}
	#[test]
	fn ex3_p2() {
		assert_eq!(super::part_2("\n17,x,13,19\n").unwrap(), 3417);
	}
	#[test]
	fn ex4_p2() {
		assert_eq!(super::part_2("\n67,7,59,61\n").unwrap(), 754018);
	}
	#[test]
	fn ex5_p2() {
		assert_eq!(super::part_2("\n67,x,7,59,61\n").unwrap(), 779210);
	}
	#[test]
	fn ex6_p2() {
		assert_eq!(super::part_2("\n67,7,x,59,61\n").unwrap(), 1261476);
	}
	#[test]
	fn ex7_p2() {
		assert_eq!(super::part_2("\n1789,37,47,1889\n").unwrap(), 1202161486);
	}
}

fn part_1(input: &str) -> Result<i64> {
	let mut lines = input.lines();
	let earliest_timestamp = lines
		.next()
		.ok_or_else(|| anyhow!("bad input"))?
		.parse::<i64>()?;
	let buses = lines
		.next()
		.map(|line| {
			line.split(",")
				.filter_map(|c| c.parse::<i64>().ok())
				.collect_vec()
		})
		.ok_or_else(|| anyhow!("bad input"))?;

	let p1 = buses
		.clone()
		.into_iter()
		.map(|x| {
			let rounds = earliest_timestamp / x;
			(x, x * (rounds + 1))
		})
		.map(|(x, x_time)| (x, x_time - earliest_timestamp))
		.min_by_key(|(_, x_time)| *x_time)
		.unwrap();

	Ok(p1.0 * p1.1)
}

fn part_2(input: &str) -> Result<i64> {
	let departure = input
		.lines()
		.skip(1)
		.next()
		.ok_or_else(|| anyhow!("bad input"))?
		.split(",")
		.enumerate()
		.filter_map(|(idx, bus)| bus.parse::<i64>().ok().map(|bus| (idx as i64, bus)))
		.fold((0, 1), |(mut x, mod_x), (idx, bus)| {
			(
				loop {
					if (x + idx) % bus == 0 {
						break x;
					}
					x += mod_x;
				},
				mod_x * bus,
			)
		})
		.0;

	Ok(departure)
}
