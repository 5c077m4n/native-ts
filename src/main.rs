mod libs;

use libs::{parse_cli_args::CliArgs, remote_script, tokens::Token};
use logos::Logos;
use std::{
	io::{self, Write},
	path::PathBuf,
};
use structopt::StructOpt;
use tokio::{self, fs};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let args = CliArgs::from_args();

	if let Some(path) = args.path {
		if let Ok(remote_path) = Url::parse(path.to_str().unwrap()) {
			let script =
				remote_script::get_remote_script(PathBuf::from(remote_path.as_str())).await?;
			let lex = Token::lexer(script.as_str());

			for token in lex.spanned() {
				println!("{:?}", token);
			}
		} else {
			let file_content = fs::read_to_string(path).await.unwrap();
			let lex = Token::lexer(file_content.as_str());

			for token in lex.spanned() {
				println!("{:?}", token);
			}
		}
	} else if let Some(command_to_eval) = args.evaluate {
		let lex = Token::lexer(command_to_eval.as_str());

		for token in lex.spanned() {
			println!("{:?}", token);
		}
	} else {
		loop {
			print!("> ");
			let _ = io::stdout().flush();

			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Unable to read user input");

			let lex = Token::lexer(input.as_str());
			for token in lex.spanned() {
				println!("{:?}", token);
			}
		}
	}

	Ok(())
}
