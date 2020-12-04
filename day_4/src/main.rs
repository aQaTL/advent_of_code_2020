use itertools::Itertools;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let (part_1, part_2) = std::fs::read_to_string("day_4/input.txt")?
		.lines()
		.group_by(|line| line.is_empty())
		.into_iter()
		.map(|(_, grouped_lines)| {
			grouped_lines
				.flat_map(|line: &str| {
					line.split(" ")
						.flat_map(|pair| pair.split(":").map(ToOwned::to_owned).tuples())
				})
				.collect::<HashMap<_, _>>()
		})
		.fold((0, 0), |(part_1, part_2), hm| {
			(
				part_1 + is_valid_part_1(&hm) as u64,
				part_2 + is_valid_part_2(&hm) as u64,
			)
		});

	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2);

	Ok(())
}

fn is_valid_part_1(passport: &HashMap<String, String>) -> bool {
	passport.contains_key("byr")
		&& passport.contains_key("iyr")
		&& passport.contains_key("eyr")
		&& passport.contains_key("hgt")
		&& passport.contains_key("hcl")
		&& passport.contains_key("ecl")
		&& passport.contains_key("pid")
}

fn is_valid_part_2(passport: &HashMap<String, String>) -> bool {
	passport
		.get("byr")
		.and_then(|x| x.parse().ok())
		.map(|x: u64| x >= 1920 && x <= 2002)
		.unwrap_or_default()
		&& passport
			.get("iyr")
			.and_then(|x| x.parse().ok())
			.map(|x: u64| x >= 2010 && x <= 2020)
			.unwrap_or_default()
		&& passport
			.get("eyr")
			.and_then(|x| x.parse().ok())
			.map(|x: u64| x >= 2020 && x <= 2030)
			.unwrap_or_default()
		&& passport
			.get("hgt")
			.and_then(|x| {
				x.strip_suffix("cm")
					.and_then(|x| x.parse().ok())
					.map(|x: u64| x >= 150 && x <= 193)
					.or_else(|| {
						x.strip_suffix("in")
							.and_then(|x| x.parse().ok())
							.map(|x: u64| x >= 59 && x <= 76)
					})
			})
			.unwrap_or_default()
		&& passport
			.get("hcl")
			.and_then(|x| x.strip_prefix("#"))
			.filter(|x| x.len() == 6)
			.map(|x| {
				x.parse().ok().unwrap_or_else(|| {
					x.bytes().all(|x| match x {
						b'a'..=b'f' | b'0'..=b'9' => true,
						_ => false,
					})
				})
			})
			.unwrap_or_default()
		&& passport
			.get("ecl")
			.map(|x| match x.as_str() {
				"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
				_ => false,
			})
			.unwrap_or_default()
		&& passport
			.get("pid")
			.filter(|x| x.len() == 9)
			.map(|x| x.parse::<u64>().is_ok())
			.unwrap_or_default()
}
