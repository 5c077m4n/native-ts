#![cfg(test)]

use super::{JsToken, StaticImportToken};
use logos::Logos;

mod js_token_tests {
	use super::*;

	#[test]
	fn sanity() {
		let mut lex = JsToken::lexer(r"Some test string.");

		assert_eq!(lex.next(), Some(JsToken::Text("Some".to_string())));
		assert_eq!(lex.span(), 0..4);
		assert_eq!(lex.slice(), "Some");

		assert_eq!(lex.next(), Some(JsToken::Text("test".to_string())));
		assert_eq!(lex.span(), 5..9);
		assert_eq!(lex.slice(), "test");

		assert_eq!(lex.next(), Some(JsToken::Text("string".to_string())));
		assert_eq!(lex.span(), 10..16);
		assert_eq!(lex.slice(), "string");

		assert_eq!(lex.next(), Some(JsToken::Period));
		assert_eq!(lex.span(), 16..17);
		assert_eq!(lex.slice(), ".");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_console_log() {
		let mut lex = JsToken::lexer(r"console.log(123);");

		assert_eq!(lex.next(), Some(JsToken::Console));
		assert_eq!(lex.next(), Some(JsToken::Period));
		assert_eq!(lex.next(), Some(JsToken::Text("log".to_owned())));
		assert_eq!(lex.next(), Some(JsToken::BracketOpen));
		assert_eq!(lex.next(), Some(JsToken::Number(123.)));
		assert_eq!(lex.next(), Some(JsToken::BracketClose));
	}

	#[test]
	fn parse_number_basic_int() {
		let mut lex = JsToken::lexer(r"12");

		assert_eq!(lex.next(), Some(JsToken::Number(12.)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_neg_int() {
		let mut lex = JsToken::lexer(r"-12");

		assert_eq!(lex.next(), Some(JsToken::Number(-12.)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_basic_float() {
		let mut lex = JsToken::lexer(r"12.");

		assert_eq!(lex.next(), Some(JsToken::Number(12.)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_neg_float() {
		let mut lex = JsToken::lexer(r"-12.");

		assert_eq!(lex.next(), Some(JsToken::Number(-12.)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_exp_int() {
		let mut lex = JsToken::lexer(r"12e3");

		assert_eq!(
			lex.next(),
			Some(JsToken::Number(12_000.)),
			"Bad pattern: {:?}",
			lex.slice()
		);
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_exp_float() {
		let mut lex = JsToken::lexer(r"13.e2");

		assert_eq!(lex.next(), Some(JsToken::Number(13e2)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_float() {
		let mut lex = JsToken::lexer(r"123123.");

		assert_eq!(lex.next(), Some(JsToken::Number(123_123.)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_float_2() {
		let mut lex = JsToken::lexer(r"123123.55");

		assert_eq!(lex.next(), Some(JsToken::Number(123_123.55)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_expr() {
		let mut lex = JsToken::lexer(r#"23e3 !== 22."#);

		assert_eq!(
			lex.next(),
			Some(JsToken::Number(23_000.)),
			"Bad token: {}",
			lex.slice()
		);
		assert_eq!(lex.next(), Some(JsToken::NotEqEq));
		assert_eq!(lex.next(), Some(JsToken::Number(22.)));
		assert_eq!(lex.next(), None);
	}
}

mod static_import_token_tests {
	use super::*;

	#[test]
	fn test_default_import() {
		let mut lex = StaticImportToken::lexer("import qwerty from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(StaticImportToken::Import));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "qwerty");

		assert_eq!(lex.next(), Some(StaticImportToken::From));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn test_named_import() {
		let mut lex = StaticImportToken::lexer("import { qwerty } from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(StaticImportToken::Import));

		assert_eq!(lex.next(), Some(StaticImportToken::BracketCurlyOpen));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "qwerty");

		assert_eq!(lex.next(), Some(StaticImportToken::BracketCurlyClose));

		assert_eq!(lex.next(), Some(StaticImportToken::From));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn test_multi_named_import() {
		let mut lex =
			StaticImportToken::lexer("import { fn1, fn2 } from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(StaticImportToken::Import));

		assert_eq!(lex.next(), Some(StaticImportToken::BracketCurlyOpen));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "fn1");

		assert_eq!(lex.next(), Some(StaticImportToken::Comma));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "fn2");

		assert_eq!(lex.next(), Some(StaticImportToken::BracketCurlyClose));

		assert_eq!(lex.next(), Some(StaticImportToken::From));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn test_star_as_import() {
		let mut lex = StaticImportToken::lexer("import * as All from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(StaticImportToken::Import));

		assert_eq!(lex.next(), Some(StaticImportToken::Star));

		assert_eq!(lex.next(), Some(StaticImportToken::As));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "All");

		assert_eq!(lex.next(), Some(StaticImportToken::From));

		assert_eq!(lex.next(), Some(StaticImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}
}
