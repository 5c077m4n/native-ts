use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum StaticImportToken {
	#[token("{")]
	BracketCurlyOpen,
	#[token("}")]
	BracketCurlyClose,

	#[token("import ")]
	Import,
	#[token("export ")]
	Export,
	#[token(" from ")]
	From,
	#[token(" as ")]
	As,

	#[token("*")]
	Star,
	#[token(".")]
	Period,
	#[token(",")]
	Comma,
	#[token(";")]
	SemiColon,
	#[token(":")]
	Colon,

	#[regex(r#"[a-zA-Z0-9$_'/".]+"#)]
	Text,
	#[regex(r"[\s\t\n\f]+", logos::skip)]
	#[error]
	Error,
}

#[cfg(test)]
mod import_token_tests {
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
