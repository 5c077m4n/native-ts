use exitcode;
use reqwest::{self, StatusCode};
use std::process;

pub async fn get_remote_script(url: &str) -> Result<String, reqwest::Error> {
	if !url.ends_with(".ts") {
		process::exit(exitcode::OSFILE);
	}

	let response = reqwest::get(url).await?;
	assert_eq!(response.status(), StatusCode::OK, "Could not get {:?}", url);

	response.text().await
}

#[cfg(test)]
mod remote_script_tests {
	use super::*;
	use tokio::runtime::Runtime;

	#[test]
	#[should_panic]
	fn incorrect_remote_file_type() {
		Runtime::new()
			.unwrap()
			.block_on(get_remote_script(
				"https://github.com/5c077m4n/http-responder/blob/master/src/index.txt",
			))
			.unwrap();
	}

	#[test]
	#[should_panic]
	fn local_file() {
		Runtime::new()
			.unwrap()
			.block_on(get_remote_script("./src/index.js"))
			.unwrap();
	}

	#[test]
	fn should_work() {
		Runtime::new().unwrap().block_on(async {
			let script = get_remote_script(
				"https://github.com/5c077m4n/http-responder/blob/master/src/index.ts",
			)
			.await
			.unwrap();

			assert!(script.len() > 0);
		});
	}
}
