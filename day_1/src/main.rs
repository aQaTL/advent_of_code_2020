fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_1/input.txt")?
		.lines()
		.map(|x| x.parse::<u64>().unwrap())
		.collect::<Vec<_>>();

	let part_1 = input
		.iter()
		.map(|a| (a, input.iter()))
		.find_map(|(&a, mut b)| b.find(|&&b| b + a == 2020).map(|b| b * a))
		.unwrap();

	println!("Part 1: {}", part_1);

	let part_2 = input
		.iter()
		.map(|a| (a, input.iter().map(|b| (b, input.iter()))))
		.find_map(|(&a, mut b)| {
			b.find_map(|(&b, mut c)| c.find(|&&c| c + b + a == 2020).map(|c| c * b * a))
		})
		.unwrap();

	println!("Part 2: {}", part_2);

	Ok(())
}
