use std::path::PathBuf;
use tokio::fs;

#[allow(dead_code)]
pub async fn read_content(file_path: PathBuf) -> String {
	fs::read_to_string(file_path).await.expect("File not found")
}

#[cfg(test)]
mod tests {
	extern crate tokio;
	use super::*;
	use tokio::runtime::Runtime;

	#[test]
	#[should_panic(expected = "File not found")]
	fn sanity() {
		Runtime::new()
			.unwrap()
			.block_on(read_content(PathBuf::from("./no/such/file.txt")));
	}
}
