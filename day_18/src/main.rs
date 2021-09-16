use anyhow::Result;
use nom::branch::alt;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use std::num::ParseIntError;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_18/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> i64 {
	input
		.lines()
		.filter_map(|expr| parse_expr(expr).map(|(_input, expr)| expr).ok())
		.fold(0, |sum, expr| sum + expr.eval())
}

fn part_2(input: &str) -> i64 {
	input
		.lines()
		.filter_map(|expr| parse_expr_p2(expr).map(|(_input, expr)| expr).ok())
		.fold(0, |sum, expr| sum + expr.eval())
}

#[derive(Debug)]
struct Expr {
	vec: Vec<(Operation, Operand)>,
}

#[derive(Copy, Clone, Debug)]
enum Operation {
	Plus,
	Multiply,
}

#[derive(Debug)]
enum Operand {
	Number(i64),
	Expr(Expr),
}

impl Expr {
	fn eval(&self) -> i64 {
		let mut result = 0;
		for (operation, operand) in &self.vec {
			let v = match operand {
				Operand::Number(v) => *v,
				Operand::Expr(expr) => expr.eval(),
			};
			match operation {
				Operation::Plus => result += v,
				Operation::Multiply => result *= v,
			}
		}
		result
	}
}

fn parse_expr(mut input: &str) -> IResult<&str, Expr> {
	let mut expr = Vec::new();

	let mut operation = Operation::Plus;

	loop {
		match parse_operand(input) {
			Ok((tail, operand)) => {
				input = tail;
				expr.push((operation, operand));
			}
			Err(_) => match parse_operation(input) {
				Ok((tail, new_operation)) => {
					input = tail;
					operation = new_operation;
				}
				Err(_) => break,
			},
		}
	}

	Ok((input, Expr { vec: expr }))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
	alt((
		map(
			map_res(
				terminated(digit1, multispace0),
				|digit: &str| -> Result<i64, ParseIntError> { digit.parse() },
			),
			Operand::Number,
		),
		map(
			terminated(delimited(char('('), parse_expr, char(')')), multispace0),
			Operand::Expr,
		),
	))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
	delimited(
		multispace0,
		alt((
			map(char('+'), |_| Operation::Plus),
			map(char('*'), |_| Operation::Multiply),
		)),
		multispace0,
	)(input)
}

fn parse_expr_p2(mut input: &str) -> IResult<&str, Expr> {
	let mut expr = Vec::new();

	let mut operation = Operation::Plus;

	loop {
		match parse_operand_p2(input) {
			Ok((tail, operand)) => {
				input = tail;
				expr.push((operation, operand));
			}
			Err(_) => match parse_operation(input) {
				Ok((tail, new_operation)) => {
					input = tail;
					match new_operation {
						Operation::Multiply => {
							let (tail, new_expr) = parse_expr_p2(input)?;
							input = tail;
							expr.push((new_operation, Operand::Expr(new_expr)));
						}
						Operation::Plus => {
							operation = new_operation;
						}
					}
				}
				Err(_) => break,
			},
		}
	}

	Ok((input, Expr { vec: expr }))
}

fn parse_operand_p2(input: &str) -> IResult<&str, Operand> {
	alt((
		map(
			map_res(
				terminated(digit1, multispace0),
				|digit: &str| -> Result<i64, ParseIntError> { digit.parse() },
			),
			Operand::Number,
		),
		map(
			terminated(delimited(char('('), parse_expr_p2, char(')')), multispace0),
			Operand::Expr,
		),
	))(input)
}

#[cfg(test)]
mod tests {
	const EXAMPLES: [(&str, i64, i64); 6] = [
		("1 + 2 * 3 + 4 * 5 + 6", 71, 231),
		("1 + (2 * 3) + (4 * (5 + 6))", 51, 51),
		("2 * 3 + (4 * 5)", 26, 46),
		("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437, 1445),
		("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
		(
			"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
			13632,
			23340,
		),
	];

	#[test]
	fn part_1() {
		for (example, expected, _) in EXAMPLES {
			assert_eq!(super::part_1(example), expected);
		}
	}

	#[test]
	fn part_2() {
		for (example, _, expected) in EXAMPLES {
			assert_eq!(super::part_2(example), expected);
		}
	}
}
