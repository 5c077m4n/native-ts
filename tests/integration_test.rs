use std::process::Output;

mod common;

#[cfg(test)]
mod tests {
	use super::*;

	type TestResult = Result<(), Box<dyn std::error::Error>>;

	#[test]
	fn sanity() -> TestResult {
		let output: Output = common::cargo_run("-e 'console.log(123)'")?;
		assert_eq!(output.status.code(), Some(0));

		Ok(())
	}

	#[test]
	#[ignore]
	fn bad_local_file() -> TestResult {
		let output: Output = common::cargo_run("--path ./no/such/file.ts")?;
		assert_eq!(output.status.code(), Some(1));

		Ok(())
	}

	#[test]
	#[ignore]
	fn bad_file_ext() -> TestResult {
		let output = common::cargo_run(
			"--path https://github.com/5c077m4n/http-responder/blob/master/src/index.txt",
		)?;
		assert_eq!(output.status.code(), Some(1));

		Ok(())
	}

	#[test]
	fn all_good_local_file() -> TestResult {
		let output = common::cargo_run("--path ./assets/test-1.ts")?;
		assert_eq!(output.status.code(), Some(0));

		Ok(())
	}

	#[test]
	fn all_good_remote_file() -> TestResult {
		let output = common::cargo_run(
			"--path https://github.com/5c077m4n/http-responder/blob/master/src/index.ts",
		)?;
		assert_eq!(output.status.code(), Some(0));

		Ok(())
	}
}
