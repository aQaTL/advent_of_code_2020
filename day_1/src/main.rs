fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_1/input.txt")?
		.lines()
		.map(|x| x.parse::<u64>().unwrap())
		.collect::<Vec<_>>();

	let part_1 = input
		.iter()
		.enumerate()
		.map(|(a_idx, a)| ((a_idx, a), input.iter()))
		.find_map(|((a_idx, &a), b)| b.skip(a_idx).find(|&&b| b + a == 2020).map(|b| b * a))
		.unwrap();

	println!("Part 1: {}", part_1);

	let part_2 = input
		.iter()
		.enumerate()
		.map(|(a_idx, a)| {
			(
				(a_idx, a),
				input
					.iter()
					.enumerate()
					.map(|(b_idx, b)| ((b_idx, b), input.iter())),
			)
		})
		.find_map(|((a_idx, &a), b)| {
			b.skip(a_idx).find_map(|((b_idx, &b), c)| {
				c.skip(b_idx)
					.find(|&&c| c + b + a == 2020)
					.map(|c| c * b * a)
			})
		})
		.unwrap();

	println!("Part 2: {}", part_2);

	Ok(())
}
