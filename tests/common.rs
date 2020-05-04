#![cfg(test)]
#![allow(dead_code)]

use std::io;
use std::process::{Command, Output};
use tokio::runtime::Runtime;

pub fn setup() -> Result<Runtime, io::Error> {
	Runtime::new()
}

pub fn exec(args: &[&str]) -> io::Result<Output> {
	if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/C").args(args).output()
	} else {
		Command::new("sh").arg("-c").args(args).output()
	}
}

pub fn cargo_run(args: &[&str]) -> io::Result<Output> {
	exec(&[&["cargo", "run", "--"], args].concat())
}
