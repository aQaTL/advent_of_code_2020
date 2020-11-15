use anyhow::Context;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use thiserror::Error;
use ureq::Cookie;

static AOC_ADDR: &str = "https://adventofcode.com";

#[derive(Deserialize)]
struct AocCfg {
	year: u32,
	session_cookie: String,
}

#[derive(Debug, Error)]
pub enum CargoTomlParserError {
	#[error("failed to get \"workspace\" section")]
	FailedToGetWorkspace,
	#[error("failed to get \"members\" array")]
	FailedToGetMembers,
}

fn main() -> anyhow::Result<()> {
	let cfg: AocCfg =
		toml::from_slice(&std::fs::read("aoc_cfg.toml").context("failed to read aoc_cfg.toml")?)
			.context("failed to parse aoc_cfg.toml")?;

	let mut cargo_workspace: toml::Value =
		toml::from_slice(&std::fs::read("Cargo.toml").context("failed to read Cargo.toml")?)
			.context("failed to parse Cargo.toml")?;

	let day = add_new_day_to_workspace(&mut cargo_workspace)?;

	println!("Next day: {}", day);
	println!("Adding \"day_{}\" to workspace members", day);

	let mut cargo_workspace_file = File::create("Cargo.toml")?;
	let cargo_workspace_str = toml::to_string_pretty(&cargo_workspace)?;
	cargo_workspace_file.write_all(cargo_workspace_str.as_bytes())?;

	let new_crate_name = format!("day_{}", day);

	println!("Creating new crate (\"{}\")", new_crate_name);

	let cargo_new_status = Command::new("cargo")
		.args(&["new", &new_crate_name])
		.status()?;
	if !cargo_new_status.success() {
		return Err(anyhow::anyhow!(
			"cargo new failed with code {}",
			cargo_new_status.code().unwrap_or_default()
		));
	}

	std::env::set_current_dir(&new_crate_name)?;

	println!("Downloading day {} input...", day);

	let day_1_input = get_input(cfg.year, day, &cfg.session_cookie)?;

	let mut input_file = File::create("input.txt")?;
	input_file.write_all(day_1_input.as_bytes())?;

	println!("Input saved to {}/input.txt", new_crate_name);

	Ok(())
}

fn add_new_day_to_workspace(cargo_workspace: &mut toml::Value) -> anyhow::Result<u32> {
	let workspace = cargo_workspace
		.get_mut("workspace")
		.ok_or(CargoTomlParserError::FailedToGetWorkspace)?;
	let members = workspace
		.get_mut("members")
		.ok_or(CargoTomlParserError::FailedToGetMembers)?
		.as_array_mut()
		.ok_or(CargoTomlParserError::FailedToGetMembers)?;

	let day = find_next_day(members);
	members.push(toml::Value::String(format!("day_{}", day)));
	Ok(day)
}

fn find_next_day(members: &Vec<toml::Value>) -> u32 {
	members
		.iter()
		.filter_map(toml::Value::as_str)
		.filter_map(|str| str.strip_prefix("day_"))
		.filter_map(|day| day.parse::<u32>().ok())
		.max()
		.unwrap_or(0)
		+ 1
}

fn get_input(year: u32, day: u32, cookie: &str) -> anyhow::Result<String> {
	let cookie = Cookie::build("session", cookie)
		.domain("adventofcode.com")
		.path("/")
		.secure(true)
		.finish()
		.into_owned();

	let agent = ureq::agent();
	agent.set_cookie(cookie);

	let mut request = agent.get(&format!(
		"{addr}/{year}/day/{day}/input",
		addr = AOC_ADDR,
		year = year,
		day = day
	));
	let response = request.call();

	if response.synthetic() {
		// SAFETY: synthetic returns true only when error is Some
		return Err(response.into_synthetic_error().unwrap().into());
	}

	if response.error() {
		return Err(anyhow::anyhow!(
			"Failed to fetch input: {}",
			response.status_line()
		));
	}

	response.into_string().map_err(Into::into)
}
