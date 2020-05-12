use logos::{self, Logos};

#[derive(Logos, Debug, PartialEq)]
pub enum ImportToken {
	#[token("{")]
	BracketCurlyOpen,
	#[token("}")]
	BracketCurlyClose,

	#[token("import ")]
	Import,
	#[token("export ")]
	Export,
	#[token("from ")]
	From,
	#[token("as")]
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
		let mut lex = ImportToken::lexer("import qwerty from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(ImportToken::Import));
		assert_eq!(lex.slice(), "import ");

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "qwerty");

		assert_eq!(lex.next(), Some(ImportToken::From));
		assert_eq!(lex.slice(), "from ");

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn test_named_import() {
		let mut lex = ImportToken::lexer("import { qwerty } from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(ImportToken::Import));
		assert_eq!(lex.slice(), "import ");

		assert_eq!(lex.next(), Some(ImportToken::BracketCurlyOpen));

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "qwerty");

		assert_eq!(lex.next(), Some(ImportToken::BracketCurlyClose));

		assert_eq!(lex.next(), Some(ImportToken::From));
		assert_eq!(lex.slice(), "from ");

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn test_multi_named_import() {
		let mut lex = ImportToken::lexer("import { fn1, fn2 } from './path/to/local/file.js'");

		assert_eq!(lex.next(), Some(ImportToken::Import));
		assert_eq!(lex.slice(), "import ");

		assert_eq!(lex.next(), Some(ImportToken::BracketCurlyOpen));

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "fn1");

		assert_eq!(lex.next(), Some(ImportToken::Comma));

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "fn2");

		assert_eq!(lex.next(), Some(ImportToken::BracketCurlyClose));

		assert_eq!(lex.next(), Some(ImportToken::From));
		assert_eq!(lex.slice(), "from ");

		assert_eq!(lex.next(), Some(ImportToken::Text));
		assert_eq!(lex.slice(), "'./path/to/local/file.js'");

		assert_eq!(lex.next(), None);
	}
}
