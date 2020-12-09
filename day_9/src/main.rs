use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input: Vec<usize> = std::fs::read_to_string("day_9/input.txt")?
		.lines()
		.map(str::parse)
		.try_collect()?;

	let preamble_size = 25;

	let invalid_number = input
		.iter()
		.copied()
		.enumerate()
		.skip(preamble_size)
		.find(|(idx, x)| {
			!input[(idx - preamble_size)..=(idx - 1)]
				.iter()
				.copied()
				.combinations(2)
				.any(|comb| comb.into_iter().sum::<usize>() == *x)
		})
		.map(|(_, x)| x)
		.ok_or_else(|| anyhow::anyhow!("Part 1 fail"))?;
	println!("Part 1: {}", invalid_number);

	let part_2 = (2..input.len())
		.into_iter()
		.find_map(|i| {
			input
				.as_slice()
				.windows(i)
				.find(|x| x.iter().sum::<usize>() == invalid_number)
				.and_then(|x| x.iter().minmax().into_option().map(|(min, max)| min + max))
		})
		.ok_or_else(|| anyhow::anyhow!("Part 2 fail"))?;
	println!("Part 2: {}", part_2);

	Ok(())
}
