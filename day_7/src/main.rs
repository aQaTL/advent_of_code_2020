use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Bag = (String, u64);

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_7/input.txt")?;

	let re = Regex::new(
		r"^(?P<bag>\w+ \w+) bags contain |(?:(?P<contains>no other bags.)|(?P<quantity>\d+) (?P<inside_bag_name>\w+ \w+) bag(?:s)?(?:, )?)",
	)?;

	let mut bags = HashMap::<String, Vec<Bag>>::new();
	for line in input.lines() {
		let captures = re.captures_iter(line).collect::<Vec<_>>();

		let bag = captures[0].name("bag").unwrap().as_str().to_owned();
		let contains_bags = captures[1].name("contains").is_none();
		if contains_bags {
			for group in captures.into_iter().skip(1) {
				let quantity = group
					.name("quantity")
					.unwrap()
					.as_str()
					.parse::<u64>()
					.unwrap();
				let inside_bag = group.name("inside_bag_name").unwrap().as_str().to_owned();
				bags.entry(bag.clone())
					.or_default()
					.push((inside_bag, quantity));
			}
		}
	}

	let mut part_1 = 0;
	for (_, bags_inside) in bags.iter() {
		let mut search_queue: Vec<&Bag> = Vec::new();
		let mut searched: Vec<&str> = Vec::new();
		search_queue.extend(bags_inside);
		let mut this_shiny_gold_count = 0;
		while let Some((bag_name, _)) = search_queue.pop() {
			if !searched.contains(&bag_name.as_str()) {
				if bag_name == "shiny gold" {
					this_shiny_gold_count += 1;
				} else {
					if let Some(v) = bags.get(bag_name) {
						search_queue.extend(v);
					}
					searched.push(bag_name);
				}
			}
		}
		if this_shiny_gold_count > 0 {
			part_1 += 1;
		}
	}
	println!("Part 1: {}", part_1);

	let mut part_2: u64 = 0;
	let mut search_queue: Vec<Bag> = Vec::new();
	search_queue.extend(bags["shiny gold"].clone());
	while let Some((bag_name, quantity)) = search_queue.pop() {
		part_2 += quantity;
		if let Some(v) = bags.get(&bag_name) {
			search_queue.extend(
				v.clone()
					.into_iter()
					.update(|(_, v_quantity)| *v_quantity *= quantity),
			);
		}
	}
	println!("Part 2: {}", part_2);

	Ok(())
}
