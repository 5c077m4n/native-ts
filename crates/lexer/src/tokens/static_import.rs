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
