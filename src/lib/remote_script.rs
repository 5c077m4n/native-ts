use reqwest;
use std::io::{Error, ErrorKind, Result};

pub async fn get_remote_script(url: &str) -> Result<String> {
	if !url.ends_with(".ts") && !url.ends_with(".js") {
		return Err(Error::new(ErrorKind::InvalidInput, "Bad file type."));
	}

	let response = reqwest::get(url).await.unwrap();
	if response.status() != reqwest::StatusCode::OK {
		return Err(Error::new(
			ErrorKind::NotFound,
			format!("Could not fetch {:?}", url),
		));
	}

	Ok(response.text().await.unwrap())
}

#[cfg(test)]
mod remote_script_tests {
	use super::*;

	#[tokio::test]
	#[should_panic]
	async fn incorrect_remote_file_type() {
		let _ = get_remote_script(
			"https://raw.githubusercontent.com/5c077m4n/http-responder/master/src/index.txt",
		)
		.await
		.unwrap();
	}

	#[tokio::test]
	#[should_panic]
	async fn local_file() {
		let _ = get_remote_script("./src/index.js").await.unwrap();
	}

	#[tokio::test]
	async fn should_work() {
		let script = get_remote_script(
			"https://raw.githubusercontent.com/5c077m4n/http-responder/master/src/index.ts",
		)
		.await
		.unwrap();

		assert!(script.len() > 0);
	}
}
