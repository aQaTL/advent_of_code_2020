use anyhow::Result;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{char, digit1, line_ending, multispace0, multispace1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_16/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> Result<u64> {
	let document = parse_document(input)?;
	let mut error_scanning_rate = 0;
	for nearby_ticket in &document.nearby_tickets {
		for rule_value in &nearby_ticket.0 {
			if !document
				.rules
				.iter()
				.any(|rule| rule.ranges.iter().any(|range| range.contains(rule_value)))
			{
				error_scanning_rate += rule_value;
			}
		}
	}

	Ok(error_scanning_rate)
}

fn part_2(input: &str) -> Result<u64> {
	let document = parse_document(input)?;

	let mut ticket_fields = Vec::<Vec<u64>>::new();

	for field_idx in 0..document.rules.len() {
		'tickets: for ticket in &document.nearby_tickets {
			for rule_value in &ticket.0 {
				if !document
					.rules
					.iter()
					.any(|rule| rule.ranges.iter().any(|range| range.contains(rule_value)))
				{
					continue 'tickets;
				}
			}
			match ticket_fields.get_mut(field_idx) {
				Some(v) => v.push(ticket.0[field_idx]),
				None => {
					ticket_fields.push(vec![ticket.0[field_idx]]);
				}
			}
		}
	}

	let mut product = 1;

	let mut visited = Vec::<usize>::new();
	let mut visited_field = Vec::<usize>::new();

	while visited.len() != document.rules.len() {
		for (rule_idx, rule) in document.rules.iter().enumerate() {
			if visited.contains(&rule_idx) {
				continue;
			}

			let mut matching_fields_count = 0;
			let mut last_matching_idx = 0;

			for (idx, columns) in ticket_fields.iter().enumerate() {
				if visited_field.contains(&idx) {
					continue;
				}

				let matches = columns
					.iter()
					.all(|column| rule.ranges.iter().any(|range| range.contains(column)));

				if matches {
					matching_fields_count += 1;
					last_matching_idx = idx;
				}
			}
			if matching_fields_count == 1 {
				visited.push(rule_idx);
				visited_field.push(last_matching_idx);
				if rule.name.starts_with("departure") {
					product *= document.my_ticket.0[last_matching_idx];
				}
			}
		}
	}

	Ok(product)
}

#[derive(Debug)]
struct Document<'a> {
	rules: Vec<Rule<'a>>,
	my_ticket: Ticket,
	nearby_tickets: Vec<Ticket>,
}

#[derive(Debug)]
struct Rule<'a> {
	name: &'a str,
	ranges: Vec<RangeInclusive<u64>>,
}

#[derive(Debug)]
struct Ticket(Vec<u64>);

fn parse_document(input: &str) -> Result<Document> {
	match parse_document_nom(input) {
		Ok((_, document)) => Ok(document),
		Err(e) => Err(anyhow::anyhow!("{:?}", e)),
	}
}

fn parse_document_nom(input: &str) -> IResult<&str, Document> {
	let (input, rules) = separated_list1(multispace1, parse_rule)(input)?;

	let (input, _) = preceded(multispace0, tag("your ticket:\n"))(input)?;
	let (input, my_ticket) = parse_ticket(input)?;

	let (input, _) = preceded(multispace0, tag("nearby tickets:\n"))(input)?;
	let (input, nearby_tickets) = separated_list1(line_ending, parse_ticket)(input)?;

	Ok((
		input,
		Document {
			rules,
			my_ticket,
			nearby_tickets,
		},
	))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
	let (input, name) = take_until1(": ")(input)?;
	let (input, _) = tag(": ")(input)?;
	let (input, ranges) = separated_list1(tag(" or "), parse_range)(input)?;
	Ok((input, Rule { name, ranges }))
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
	map_res(
		separated_pair(digit1, char('-'), digit1),
		|(a, b): (&str, &str)| -> Result<RangeInclusive<u64>, std::num::ParseIntError> {
			Ok(a.parse::<u64>()?..=b.parse::<u64>()?)
		},
	)(input)
}

fn parse_ticket(input: &str) -> IResult<&str, Ticket> {
	map(
		separated_list1(
			char(','),
			map_res(digit1, |digit: &str| digit.parse::<u64>()),
		),
		Ticket,
	)(input)
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), 71);
	}

	const EXAMPLE_2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";
	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE_2).unwrap(), 1);
	}
}
