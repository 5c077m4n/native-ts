use assert_cmd::Command;

mod common;

#[cfg(test)]
mod integration_tests {
	use super::*;

	#[test]
	fn sanity() {
		Command::cargo_bin("native-ts")
			.unwrap()
			.arg("-e 'console.log(123)'")
			.assert()
			.success();
	}

	#[test]
	fn bad_local_file() {
		Command::cargo_bin("native-ts")
			.unwrap()
			.arg("./no/such/file.ts")
			.assert()
			.failure();
	}

	#[test]
	fn bad_file_ext() {
		Command::cargo_bin("native-ts")
			.unwrap()
			.arg("https://github.com/5c077m4n/http-responder/blob/master/src/index.txt")
			.assert()
			.failure();
	}

	#[test]
	fn all_good() {
		Command::cargo_bin("native-ts")
			.unwrap()
			.arg("https://github.com/5c077m4n/http-responder/blob/master/src/index.ts")
			.assert()
			.success();
	}
}
