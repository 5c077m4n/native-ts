use reqwest::{self, StatusCode};

pub async fn get_remote_script(url: &str) -> Result<String, reqwest::Error> {
	assert!(
		url.ends_with(".ts") || url.ends_with(".js"),
		"The file does not have the correct extension."
	);

	let response = reqwest::get(url).await?;
	assert_eq!(
		response.status(),
		StatusCode::OK,
		"Could not fetch {:?}",
		url
	);

	response.text().await
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
