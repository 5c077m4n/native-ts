#![cfg(test)]

use super::*;

#[cfg(test)]
mod parser_tests {
	use super::*;

	#[tokio::test]
	#[should_panic(expected = "There was an error in parsing the input `😂` @ 0..4.")]
	async fn sanity() {
		parse("😂").await.unwrap();
	}
}
