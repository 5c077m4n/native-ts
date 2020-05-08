#![cfg(test)]

mod common;

mod integration_tests {
	use super::*;
	use std::process::Output;

	type TestResult = Result<(), Box<dyn std::error::Error>>;

	#[test]
	fn sanity() -> TestResult {
		let output: Output = common::cargo_run(&["-e", "console.log(123)"])?;
		assert!(output.status.success());

		Ok(())
	}

	// TODO: fix test failing for linux - the returned error code is wrong
	#[test]
	#[ignore]
	fn bad_local_file() -> TestResult {
		let output: Output = common::cargo_run(&["--path", "no/such/file.ts"])?;
		assert!(!output.status.success());

		Ok(())
	}

	// TODO: fix test failing for linux - the returned error code is wrong
	#[test]
	#[ignore]
	fn bad_file_ext() -> TestResult {
		let output: Output = common::cargo_run(&[
			"--path",
			"https://github.com/5c077m4n/http-responder/blob/master/src/index.txt",
		])?;
		assert!(!output.status.success());

		Ok(())
	}

	#[test]
	fn all_good_local_file() -> TestResult {
		let output: Output = common::cargo_run(&["--path", "tests/assets/test-1.ts"])?;
		assert!(output.status.success());

		Ok(())
	}

	#[test]
	fn all_good_remote_file() -> TestResult {
		let output: Output = common::cargo_run(&[
			"--path",
			"https://raw.githubusercontent.com/5c077m4n/http-responder/master/src/index.ts",
		])?;
		assert!(output.status.success());

		Ok(())
	}
}
