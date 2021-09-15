use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Vec3 {
	x: i64,
	y: i64,
	z: i64,
}

impl From<(i64, i64, i64)> for Vec3 {
	fn from((x, y, z): (i64, i64, i64)) -> Self {
		Vec3 { x, y, z }
	}
}

impl From<(i64, i64)> for Vec3 {
	fn from((x, y): (i64, i64)) -> Self {
		Vec3 { x, y, z: 1 }
	}
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Vec4 {
	x: i64,
	y: i64,
	z: i64,
	w: i64,
}

impl From<(i64, i64)> for Vec4 {
	fn from((x, y): (i64, i64)) -> Self {
		Vec4 { x, y, z: 1, w: 1 }
	}
}

impl From<(i64, i64, i64, i64)> for Vec4 {
	fn from((x, y, z, w): (i64, i64, i64, i64)) -> Self {
		Vec4 { x, y, z, w }
	}
}

#[derive(Copy, Clone)]
enum State {
	Active,
	Inactive,
}

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_17/input.txt")?;
	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> usize {
	let mut dimension = parse_input::<Vec3>(input);

	let mut new_dimension = dimension.clone();

	for _ in 0..6 {
		let (min_x, max_x) = dimension
			.keys()
			.minmax_by_key(|coord| coord.x)
			.into_option()
			.map(|(min, max)| (min.x, max.x))
			.unwrap();
		let (min_y, max_y) = dimension
			.keys()
			.minmax_by_key(|coord| coord.y)
			.into_option()
			.map(|(min, max)| (min.y, max.y))
			.unwrap();
		let (min_z, max_z) = dimension
			.keys()
			.minmax_by_key(|coord| coord.z)
			.into_option()
			.map(|(min, max)| (min.z, max.z))
			.unwrap();

		for x in (min_x - 1)..=(max_x + 1) {
			for y in (min_y - 1)..=(max_y + 1) {
				for z in (min_z - 1)..=(max_z + 1) {
					let coord: Vec3 = (x, y, z).into();
					let state = dimension.get(&coord).unwrap_or(&State::Inactive);

					let active = count_active_neighbours(coord, &dimension);
					let new_state = new_dimension.entry(coord).or_insert(State::Inactive);
					match *state {
						State::Active if active == 2 || active == 3 => *new_state = State::Active,
						State::Active => *new_state = State::Inactive,
						State::Inactive if active == 3 => *new_state = State::Active,
						State::Inactive => *new_state = State::Inactive,
					}
				}
			}
		}

		std::mem::swap(&mut dimension, &mut new_dimension);
	}

	dimension
		.values()
		.filter(|state| matches!(state, State::Active))
		.count()
}

fn count_active_neighbours(coord: Vec3, dimension: &HashMap<Vec3, State>) -> usize {
	let deltas = [
		(-1, -1, -1),
		(-1, -1, 0),
		(-1, -1, 1),
		(-1, 0, -1),
		(-1, 0, 0),
		(-1, 0, 1),
		(-1, 1, 0),
		(-1, 1, -1),
		(-1, 1, 1),
		(0, -1, -1),
		(0, -1, 0),
		(0, -1, 1),
		(0, 0, -1),
		(0, 0, 1),
		(0, 1, -1),
		(0, 1, 0),
		(0, 1, 1),
		(1, -1, -1),
		(1, -1, 0),
		(1, -1, 1),
		(1, 0, -1),
		(1, 0, 0),
		(1, 0, 1),
		(1, 1, -1),
		(1, 1, 0),
		(1, 1, 1),
	];

	deltas
		.iter()
		.map(|(dx, dy, dz)| Vec3::from((coord.x + dx, coord.y + dy, coord.z + dz)))
		.map(|coord| dimension.get(&coord))
		.filter(|state| matches!(state, Some(State::Active)))
		.count()
}

fn part_2(input: &str) -> usize {
	let mut dimension = parse_input::<Vec4>(input);

	let mut new_dimension = dimension.clone();

	for _ in 0..6 {
		let (min_x, max_x) = dimension
			.keys()
			.minmax_by_key(|coord| coord.x)
			.into_option()
			.map(|(min, max)| (min.x, max.x))
			.unwrap();
		let (min_y, max_y) = dimension
			.keys()
			.minmax_by_key(|coord| coord.y)
			.into_option()
			.map(|(min, max)| (min.y, max.y))
			.unwrap();
		let (min_z, max_z) = dimension
			.keys()
			.minmax_by_key(|coord| coord.z)
			.into_option()
			.map(|(min, max)| (min.z, max.z))
			.unwrap();
		let (min_w, max_w) = dimension
			.keys()
			.minmax_by_key(|coord| coord.w)
			.into_option()
			.map(|(min, max)| (min.w, max.w))
			.unwrap();

		for x in (min_x - 1)..=(max_x + 1) {
			for y in (min_y - 1)..=(max_y + 1) {
				for z in (min_z - 1)..=(max_z + 1) {
					for w in (min_w - 1)..=(max_w + 1) {
						let coord: Vec4 = (x, y, z, w).into();
						let state = dimension.get(&coord).unwrap_or(&State::Inactive);

						let active = count_active_neighbours_4d(coord, &dimension);
						let new_state = new_dimension.entry(coord).or_insert(State::Inactive);
						match *state {
							State::Active if active == 2 || active == 3 => {
								*new_state = State::Active
							}
							State::Active => *new_state = State::Inactive,
							State::Inactive if active == 3 => *new_state = State::Active,
							State::Inactive => *new_state = State::Inactive,
						}
					}
				}
			}
		}

		std::mem::swap(&mut dimension, &mut new_dimension);
	}

	dimension
		.values()
		.filter(|state| matches!(state, State::Active))
		.count()
}

fn count_active_neighbours_4d(coord: Vec4, dimension: &HashMap<Vec4, State>) -> u8 {
	let mut active = 0;
	for x in -1..=1 {
		for y in -1..=1 {
			for z in -1..=1 {
				for w in -1..=1 {
					if x == 0 && y == 0 && z == 0 && w == 0 {
						continue;
					}

					if let Some(State::Active) =
						dimension.get(&(coord.x + x, coord.y + y, coord.z + z, coord.w + w).into())
					{
						active += 1;
					}
				}
			}
		}
	}
	active
}

fn parse_input<T>(input: &str) -> HashMap<T, State>
where
	T: From<(i64, i64)> + Eq + Hash,
{
	let mut dimension = HashMap::new();
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.bytes().enumerate() {
			match c {
				b'.' => {
					dimension.insert((x as i64, y as i64).into(), State::Inactive);
				}
				b'#' => {
					dimension.insert((x as i64, y as i64).into(), State::Active);
				}
				_ => (),
			}
		}
	}
	dimension
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = ".#.
..#
###
";

	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EXAMPLE_1), 112);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EXAMPLE_1), 848);
	}
}
