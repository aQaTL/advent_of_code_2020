fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_2/input.txt")?;
	let re = regex::Regex::new(r"(\d+)-(\d+)\s([a-z]):\s(.+)")?;

	let (mut part_1, mut part_2) = (0, 0);

	for line in input.lines() {
		let captures = re
			.captures(line)
			.ok_or(anyhow::anyhow!("regex on {} failed", line))?
			.iter()
			.filter_map(|v| v)
			.skip(1)
			.map(|cap| cap.as_str())
			.collect::<Vec<_>>();

		let low = captures[0].parse::<usize>()?;
		let high = captures[1].parse::<usize>()?;
		let letter = captures[2].as_bytes()[0];
		let password = captures[3];

		if part_1_is_valid(low, high, letter, password.as_bytes()) {
			part_1 += 1;
		}

		if part_2_is_valid(low - 1, high - 1, letter, password.as_bytes()) {
			part_2 += 1;
		}
	}

	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2);

	Ok(())
}

fn part_1_is_valid(low: usize, high: usize, letter: u8, password: &[u8]) -> bool {
	let letter_count = password.iter().filter(|&&c| c == letter).count();
	letter_count >= low && letter_count <= high
}

fn part_2_is_valid(first_idx: usize, second_idx: usize, letter: u8, password: &[u8]) -> bool {
	let mut valid = password
		.get(first_idx)
		.map(|&v| v == letter)
		.unwrap_or_default();
	if password
		.get(second_idx)
		.map(|&v| v == letter)
		.unwrap_or_default()
	{
		valid = !valid;
	}
	valid
}
