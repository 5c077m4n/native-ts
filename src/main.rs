#![forbid(unsafe_code)]

mod lib;

use lib::{parse_cli_args::CliArgs, remote_script};
use parser::parse;
use std::io::{self, Error, ErrorKind, Result, Write};
use structopt::StructOpt;
use tokio::{self, fs};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
	let args = CliArgs::from_args();

	if let Some(path) = args.path {
		if let Some(path) = path.to_str() {
			if let Ok(remote_path) = Url::parse(path) {
				let remote_script = remote_script::get_remote_script(remote_path.as_str()).await?;
				parse(&remote_script).await?;
			} else {
				let local_script = fs::read_to_string(path).await?;
				parse(&local_script).await?;
			}
		} else {
			return Err(Error::new(
				ErrorKind::InvalidInput,
				"There was an error in parsing the given file URL.",
			));
		}
	} else if let Some(command_to_eval) = args.evaluate {
		parse(&command_to_eval).await?;
	} else {
		loop {
			print!("> ");
			io::stdout().flush()?;

			let mut input = String::new();
			io::stdin().read_line(&mut input)?;

			parse(&input).await?;
		}
	}

	Ok(())
}
