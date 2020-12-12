use anyhow::Error;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_12/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let instructions: Vec<_> = input
		.lines()
		.map(|line| {
			let action = line.as_bytes()[0] as char;
			let value = line[1..].parse::<i64>()?;
			Result::<_, Error>::Ok((action, value))
		})
		.try_collect()?;

	let (mut face, mut face_idx) = ('E', 0);
	let mut point = (0, 0); //x, y

	for (action, value) in instructions {
		match action {
			'N' => point.1 -= value,
			'S' => point.1 += value,
			'E' => point.0 += value,
			'W' => point.0 -= value,
			'L' => {
				let mut new_face_idx = face_idx - value / 90;
				if new_face_idx < 0 {
					new_face_idx = 4 - -new_face_idx;
				}
				face = ['E', 'S', 'W', 'N'][new_face_idx as usize];
				face_idx = new_face_idx;
			}
			'R' => {
				let new_face_idx = (value / 90 + face_idx) % 4;
				face = ['E', 'S', 'W', 'N'][new_face_idx as usize];
				face_idx = new_face_idx;
			}
			'F' => match face {
				'N' => point.1 -= value,
				'S' => point.1 += value,
				'E' => point.0 += value,
				'W' => point.0 -= value,
				e => anyhow::bail!("unknown face {}", e),
			},
			e => anyhow::bail!("unknown action {}", e),
		}
	}

	Ok(manhattan_distance((0, 0), point))
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let instructions: Vec<_> = input
		.lines()
		.map(|line| {
			let action = line.as_bytes()[0] as char;
			let value = line[1..].parse::<i64>()?;
			Result::<_, Error>::Ok((action, value))
		})
		.try_collect()?;

	let mut ship_point = (0, 0); //x, y
	let mut waypoint = (10, -1); //x, y

	for (action, value) in instructions {
		match action {
			'N' => waypoint.1 -= value,
			'S' => waypoint.1 += value,
			'E' => waypoint.0 += value,
			'W' => waypoint.0 -= value,
			'L' => waypoint = rotate_vec3(waypoint, -value),
			'R' => waypoint = rotate_vec3(waypoint, value),
			'F' => {
				ship_point.0 += waypoint.0 * value;
				ship_point.1 += waypoint.1 * value;
			}
			e => anyhow::bail!("unknown action {}", e),
		}
	}

	Ok(manhattan_distance((0, 0), ship_point))
}

fn manhattan_distance(p: (i64, i64), q: (i64, i64)) -> i64 {
	(p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn rotate_vec3(vec2: (i64, i64), angle: i64) -> (i64, i64) {
	let (x, y) = (vec2.0 as f64, vec2.1 as f64);
	let angle = (angle as f64).to_radians();
	(
		((x * angle.cos()) - (y * angle.sin())).round() as i64,
		((x * angle.sin()) + (y * angle.cos())).round() as i64,
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	static EXAMPLE: &str = "F10
N3
F7
R90
F11
";

	#[test]
	fn ex1_p1() {
		assert_eq!(part_1(EXAMPLE).unwrap(), 25);
	}
	#[test]
	fn ex1_p2() {
		assert_eq!(part_2(EXAMPLE).unwrap(), 286);
	}

	#[test]
	fn rotating_vec() {
		let v = (10, -4);
		let expected = (4, 10);
		let angle = 90;
		assert_eq!(rotate_vec3(v, angle), expected);
	}
}
