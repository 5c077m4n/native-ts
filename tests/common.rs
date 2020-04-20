#![allow(dead_code)]

use std::io;
use tokio::runtime::Runtime;

pub fn setup() -> Result<Runtime, io::Error> {
	Runtime::new()
}
