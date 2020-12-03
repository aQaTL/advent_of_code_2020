fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_3/input.txt")?;

	let count_trees = |input: &str, x_step, y_step| {
		input
			.lines()
			.map(str::as_bytes)
			.step_by(y_step)
			.enumerate()
			.map(|(idx, line)| line[(idx * x_step) % line.len()])
			.filter(|&c| c == b'#')
			.count()
	};

	println!("Part 1: {}", count_trees(&input, 3, 1));

	let part_2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
		.iter()
		.copied()
		.map(|(x_step, y_step)| count_trees(&input, x_step, y_step))
		.product();
	println!("Part 2: {}", part_2);

	Ok(())
}
