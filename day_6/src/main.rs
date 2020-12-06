fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_6/input.txt")?;

	let mut hm = std::collections::HashMap::<u8, usize>::new();
	let (mut part_1, mut part_2) = (0, 0);
	for group in input.split("\n\n") {
		hm.clear();
		let mut people_count = 0;
		for person in group.lines() {
			for answer in person.trim().as_bytes().iter().copied() {
				*hm.entry(answer).or_insert(0) += 1;
			}
			people_count += 1;
		}
		part_1 += hm.keys().len();
		part_2 += hm
			.iter()
			.filter(|(_, answers)| **answers == people_count)
			.count();
	}

	println!("Part 1 : {}", part_1);
	println!("Part 2 : {}", part_2);
	Ok(())
}
