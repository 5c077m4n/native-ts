#![allow(dead_code)]

extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliArgs {
    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(short, long)]
    pub silent: bool,

    #[structopt(parse(from_os_str))]
    pub path: Option<PathBuf>,
}