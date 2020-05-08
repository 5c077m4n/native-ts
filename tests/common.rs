#![cfg(test)]

use std::{
	io,
	process::{Command, Output},
};

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
