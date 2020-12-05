use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let passes = std::fs::read_to_string("day_5/input.txt")?
		.lines()
		.map(str::as_bytes)
		.map(decode)
		.collect::<Vec<_>>();

	let part_1 = passes
		.iter()
		.max_by_key(|(_, _, seat_id)| *seat_id)
		.ok_or_else(|| anyhow::anyhow!("part 1 fail"))?;

	println!("Part 1: {}", part_1.2);

	let part_2 = passes
		.iter()
		.copied()
		.map(|(_, _, id)| id)
		.sorted()
		.tuple_windows::<(_, _)>()
		.find(|(id1, id2)| id1 + 2 == *id2)
		.map(|(id, _)| id + 1)
		.ok_or_else(|| anyhow::anyhow!("part 2 fail"))?;

	println!("Part 2: {}", part_2);

	Ok(())
}

fn decode(pass: &[u8]) -> (u8, u8, usize) {
	let pass = pass.iter().map(|c| match *c {
		b'F' | b'L' => 0,
		b'B' | b'R' => 1,
		_ => panic!("bad input"),
	});

	let row = binary_search(pass.clone().take(7), 0_f32..=127_f32);
	let column = binary_search(pass.skip(7), 0_f32..=7_f32);

	(row, column, (row as usize * 8) + column as usize)
}

fn binary_search(iter: impl Iterator<Item = u8>, range: std::ops::RangeInclusive<f32>) -> u8 {
	let (mut low, mut high) = range.into_inner();
	iter.fold(0, |_result, c| match c {
		0 => {
			high -= ((high - low) / 2_f32).round();
			high as u8
		}
		1 => {
			low += ((high - low) / 2_f32).round();
			low as u8
		}
		_ => panic!("bad input"),
	})
}
