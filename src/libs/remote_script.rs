use reqwest::{self, StatusCode};
use std::path::{Path, PathBuf};

pub async fn get_remote_script(url: PathBuf) -> Result<String, reqwest::Error> {
	assert!(url.to_str().unwrap().ends_with(".ts"), "Wrong file type");

	let url = Path::new(&url);
	let response = reqwest::get(url.to_str().unwrap()).await?;
	assert_eq!(response.status(), StatusCode::OK, "Could not get {:?}", url);

	response.text().await
}

#[cfg(test)]
mod remote_script_tests {
	extern crate tokio;
	use super::*;
	use tokio::runtime::Runtime;

	#[test]
	#[should_panic]
	fn incorrect_remote_file_type() {
		Runtime::new()
			.unwrap()
			.block_on(get_remote_script(PathBuf::from(
				"https://github.com/5c077m4n/http-responder/blob/master/src/index.txt",
			)))
			.unwrap();
	}

	#[test]
	#[should_panic]
	fn local_file() {
		Runtime::new()
			.unwrap()
			.block_on(get_remote_script(PathBuf::from("./src/index.js")))
			.unwrap();
	}

	#[test]
	fn should_work() {
		Runtime::new().unwrap().block_on(async {
			let script = get_remote_script(PathBuf::from(
				"https://github.com/5c077m4n/http-responder/blob/master/src/index.ts",
			))
			.await
			.unwrap();

			assert!(script.len() > 0);
		});
	}
}
