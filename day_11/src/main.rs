use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("day_11/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> Result<usize> {
	let width = input
		.lines()
		.next()
		.ok_or(anyhow!("no newlines in input"))?
		.trim()
		.len();
	let height = input.lines().count();
	let (width_signed, height_signed) = (width as i64, height as i64);
	let mut grid = input
		.bytes()
		.filter(|b| match b {
			b'L' | b'.' => true,
			_ => false,
		})
		.collect_vec();

	let mut new_grid = grid.clone();

	loop {
		let mut changes = false;
		for j in 0..height_signed {
			for i in 0..width_signed {
				if let b'.' = grid[(j as usize * width) + i as usize] {
					continue;
				}
				let mut occupied_neighbours = 0;
				for (id, jd) in &[
					(0, -1),
					(1, -1),
					(1, 0),
					(1, 1),
					(0, 1),
					(-1, -1),
					(-1, 0),
					(-1, 1),
				] {
					let (jj, ii) = (j + jd, i + id);
					if jj < 0 || ii < 0 || jj >= height_signed || ii >= width_signed {
						continue;
					}
					let (jj, ii) = (jj as usize, ii as usize);
					if let b'#' = grid[(jj * width) + ii] {
						occupied_neighbours += 1;
					}
				}
				let (j, i) = (j as usize, i as usize);
				match grid[(j * width) + i] {
					b'L' if occupied_neighbours == 0 => {
						new_grid[(j * width) + i] = b'#';
						changes = true;
					}
					b'#' if occupied_neighbours >= 4 => {
						new_grid[(j * width) + i] = b'L';
						changes = true;
					}
					e => new_grid[(j * width) + i] = e,
				}
			}
		}
		std::mem::swap(&mut grid, &mut new_grid);
		if !changes {
			break;
		}
	}

	Ok(grid.iter().filter(|&&seat| seat == b'#').count())
}

fn part_2(input: &str) -> Result<usize> {
	let width = input
		.lines()
		.next()
		.ok_or(anyhow!("no newlines in input"))?
		.trim()
		.len();
	let height = input.lines().count();
	let (width_signed, height_signed) = (width as i64, height as i64);
	let mut grid: Vec<u8> = input
		.bytes()
		.filter(|b| match b {
			b'L' | b'.' => true,
			_ => false,
		})
		.collect_vec();

	let mut new_grid = grid.clone();

	loop {
		let mut changes = false;
		for j in 0..height_signed {
			for i in 0..width_signed {
				if let b'.' = grid[(j as usize * width) + i as usize] {
					continue;
				}
				let mut occupied_neighbours = 0;

				'directions: for (id, jd) in &[
					(0, -1),
					(1, -1),
					(1, 0),
					(1, 1),
					(0, 1),
					(-1, -1),
					(-1, 0),
					(-1, 1),
				] {
					for x in 1.. {
						let (id, jd) = (id * x, jd * x);
						let (jj, ii) = (j + jd, i + id);
						if jj < 0 || ii < 0 || jj >= height_signed || ii >= width_signed {
							continue 'directions;
						}
						let (jj, ii) = (jj as usize, ii as usize);
						match grid[(jj * width) + ii] {
							b'#' => {
								occupied_neighbours += 1;
								continue 'directions;
							}
							b'L' => continue 'directions,
							_ => (),
						}
					}
				}
				let (j, i) = (j as usize, i as usize);
				match grid[(j * width) + i] {
					b'L' if occupied_neighbours == 0 => {
						new_grid[(j * width) + i] = b'#';
						changes = true;
					}
					b'#' if occupied_neighbours >= 5 => {
						new_grid[(j * width) + i] = b'L';
						changes = true;
					}
					e => new_grid[(j * width) + i] = e,
				}
			}
		}
		std::mem::swap(&mut grid, &mut new_grid);
		if !changes {
			break;
		}
	}

	Ok(grid.iter().filter(|&&seat| seat == b'#').count())
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

	#[test]
	fn part1_test() {
		assert!(matches!(part_1(EXAMPLE), Ok(37)));
	}

	#[test]
	fn part2_test() {
		assert!(matches!(part_2(EXAMPLE), Ok(26)));
	}
}
