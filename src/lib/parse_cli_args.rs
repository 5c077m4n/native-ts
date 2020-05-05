extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CliArgs {
	#[structopt(short, long)]
	pub debug: bool,

	#[structopt(short, long)]
	pub silent: bool,

	#[structopt(short, long)]
	pub evaluate: Option<String>,

	#[structopt(short, long, parse(from_os_str))]
	pub path: Option<PathBuf>,
}
