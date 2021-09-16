use anyhow::Context;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
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
	#[error("failed to get \"dependencies\" section")]
	FailedToGetDependencies,
}

#[derive(Debug)]
struct CliApp {
	day: Option<u32>,
}

impl CliApp {
	fn from_args() -> anyhow::Result<CliApp> {
		let mut day = None;

		for arg in std::env::args().skip(1) {
			if let Some(arg) = arg.strip_prefix("--day=") {
				day = Some(arg.parse::<u32>()?);
			}
		}

		Ok(CliApp { day })
	}
}

fn main() -> anyhow::Result<()> {
	let cli = CliApp::from_args()?;

	let cfg: AocCfg =
		toml::from_slice(&std::fs::read("aoc_cfg.toml").context("failed to read aoc_cfg.toml")?)
			.context("failed to parse aoc_cfg.toml")?;

	let mut cargo_workspace: toml::Value =
		toml::from_slice(&std::fs::read("Cargo.toml").context("failed to read Cargo.toml")?)
			.context("failed to parse Cargo.toml")?;

	let day = match cli.day {
		Some(specific_day) => work_with_specific_day(&mut cargo_workspace, specific_day)?,
		None => get_day_we_are_working_with(&mut cargo_workspace)?,
	};

	println!("Working with day: {}", day.get_day());

	let day_crate_name = day.get_crate_name();

	if let Day::NewDay(day) = day {
		println!("Adding \"day_{}\" to workspace members", day);

		let mut cargo_workspace_file = File::create("Cargo.toml")?;
		let cargo_workspace_str = toml::to_string_pretty(&cargo_workspace)?;
		cargo_workspace_file.write_all(cargo_workspace_str.as_bytes())?;

		create_crate_for_new_day(&day_crate_name)?;
	}

	std::env::set_current_dir(&day_crate_name)?;

	println!("Downloading day {} input...", day.get_day());

	let day_1_input = get_input(cfg.year, day.get_day(), &cfg.session_cookie)?;

	let mut input_file = File::create("input.txt")?;
	input_file.write_all(day_1_input.as_bytes())?;

	println!("Input saved to {}/input.txt", day_crate_name);

	Ok(())
}

#[derive(Copy, Clone)]
enum Day {
	NewDay(u32),
	CreatedBefore(u32),
}

impl Day {
	fn get_day(&self) -> u32 {
		match self {
			Day::NewDay(day) | Day::CreatedBefore(day) => *day,
		}
	}

	fn get_crate_name(&self) -> String {
		format!("day_{}", self.get_day())
	}
}

fn get_day_we_are_working_with(cargo_workspace: &mut toml::Value) -> anyhow::Result<Day> {
	let workspace = cargo_workspace
		.get_mut("workspace")
		.ok_or(CargoTomlParserError::FailedToGetWorkspace)?;
	let members = workspace
		.get_mut("members")
		.ok_or(CargoTomlParserError::FailedToGetMembers)?
		.as_array_mut()
		.ok_or(CargoTomlParserError::FailedToGetMembers)?;

	let mut last_day_without_input = None;
	for dir in fs::read_dir(".")?
		.filter_map(Result::ok)
		.filter(|v| v.path().is_dir())
	{
		let day = dir
			.path()
			.file_name()
			.and_then(|name| name.to_str())
			.and_then(|name| name.strip_prefix("day_"))
			.and_then(|day| day.parse::<u32>().ok());

		let day = match day {
			Some(v) => v,
			None => continue,
		};

		if !dir.path().join("input.txt").exists() {
			last_day_without_input = Some(day);
			break;
		}
	}

	match last_day_without_input {
		Some(v) => Ok(Day::CreatedBefore(v)),
		None => {
			let day = members
				.iter()
				.filter_map(toml::Value::as_str)
				.filter_map(|str| str.strip_prefix("day_"))
				.filter_map(|day| day.parse::<u32>().ok())
				.max()
				.unwrap_or(0) + 1;

			members.push(toml::Value::String(format!("day_{}", day)));
			Ok(Day::NewDay(day))
		}
	}
}

fn work_with_specific_day(
	cargo_workspace: &mut toml::Value,
	specific_day: u32,
) -> anyhow::Result<Day> {
	let workspace = cargo_workspace
		.get_mut("workspace")
		.ok_or(CargoTomlParserError::FailedToGetWorkspace)?;
	let members = workspace
		.get_mut("members")
		.ok_or(CargoTomlParserError::FailedToGetMembers)?
		.as_array_mut()
		.ok_or(CargoTomlParserError::FailedToGetMembers)?;

	let mut day_project = None;
	for dir in fs::read_dir(".")?
		.filter_map(Result::ok)
		.filter(|v| v.path().is_dir())
	{
		let day = dir
			.path()
			.file_name()
			.and_then(|name| name.to_str())
			.and_then(|name| name.strip_prefix("day_"))
			.and_then(|day| day.parse::<u32>().ok());

		match day {
			Some(day) if day == specific_day => {
				day_project = Some(day);
				break;
			}
			_ => (),
		};
	}

	match day_project {
		Some(v) => Ok(Day::CreatedBefore(v)),
		None => {
			members.push(toml::Value::String(format!("day_{}", specific_day)));
			Ok(Day::NewDay(specific_day))
		}
	}
}

fn create_crate_for_new_day(new_crate_name: &str) -> anyhow::Result<()> {
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

	add_useful_deps(&new_crate_name)?;

	Ok(())
}

fn add_useful_deps(new_crate_name: &str) -> anyhow::Result<()> {
	let cargo_toml_path = Path::new(new_crate_name).join("Cargo.toml");
	let mut cargo_toml: toml::Value = toml::from_slice(
		&std::fs::read(&cargo_toml_path)
			.with_context(|| format!("failed to read {}", cargo_toml_path.display()))?,
	)
	.with_context(|| format!("failed to parse {}", cargo_toml_path.display()))?;

	let dependencies = cargo_toml
		.get_mut("dependencies")
		.ok_or(CargoTomlParserError::FailedToGetDependencies)?
		.as_table_mut()
		.ok_or(CargoTomlParserError::FailedToGetDependencies)?;

	dependencies.insert("anyhow".to_string(), toml::Value::String("1.0".to_string()));
	dependencies.insert(
		"itertools".to_string(),
		toml::Value::String("0.9".to_string()),
	);
	dependencies.insert("regex".to_string(), toml::Value::String("1.4".to_string()));
	dependencies.insert("nom".to_string(), toml::Value::String("7.0.0".to_string()));

	let mut cargo_toml_file = File::create(&cargo_toml_path)?;
	let cargo_toml_str = toml::to_string_pretty(&cargo_toml)?;
	cargo_toml_file.write_all(cargo_toml_str.as_bytes())?;

	println!("[dependencies] updated");

	Ok(())
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

	let req_adr = format!(
		"{addr}/{year}/day/{day}/input",
		addr = AOC_ADDR,
		year = year,
		day = day
	);
	println!("Request addr: {}", req_adr);
	let mut request = agent.get(&req_adr);
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
