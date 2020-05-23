use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum BracketToken {
	#[token("(")]
	BracketOpen,
	#[token(")")]
	BracketClose,
	#[token("{")]
	BracketCurlyOpen,
	#[token("}")]
	BracketCurlyClose,
	#[token("[")]
	BracketSquareOpen,
	#[token("]")]
	BracketSquareClose,

	#[regex(r"[\s\t\n\f]+", logos::skip)]
	#[error]
	Error,
}
