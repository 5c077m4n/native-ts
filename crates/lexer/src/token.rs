use logos::{self, Lexer, Logos};

fn get_string_content(lex: &mut Lexer<Token>) -> Option<String> {
	let lex = lex.slice();
	let content: String = lex[1..lex.len() - 1].parse().ok()?;
	Some(content)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
	#[token("==")]
	EqEq,
	#[token("!=")]
	NotEq,
	#[token("===")]
	EqEqEq,
	#[token("!==")]
	NotEqEq,
	#[token("<")]
	Lt,
	#[token("<=")]
	LtEq,
	#[token(">")]
	Gt,
	#[token(">=")]
	GtEq,
	#[token("<<")]
	LShift,
	#[token(">>")]
	RShift,
	#[token(">>>")]
	ZeroFillRShift,
	#[token("+")]
	Add,
	#[token("-")]
	Sub,
	#[token("*")]
	Mul,
	#[token("/")]
	Div,
	#[token("%")]
	Mod,
	#[token("|")]
	BitOr,
	#[token("^")]
	BitXor,
	#[token("&")]
	BitAnd,
	#[token("||")]
	LogicalOr,
	#[token("&&")]
	LogicalAnd,
	#[token("**")]
	Exp,
	#[token("??")]
	NullishCoalescing,

	#[token("//")]
	Comment,
	#[token("/**")]
	CommentInlineDocStart,
	#[token("/*")]
	CommentInlineStart,
	#[token("*/")]
	CommentInlineEnd,

	#[token("=")]
	Assign,
	#[token("+=")]
	AddAssign,
	#[token("-=")]
	SubAssign,
	#[token("*=")]
	MulAssign,
	#[token("/=")]
	DivAssign,
	#[token("%=")]
	ModAssign,
	#[token("<<=")]
	LShiftAssign,
	#[token(">>=")]
	RShiftAssign,
	#[token(">>>=")]
	ZeroFillRShiftAssign,
	#[token("|=")]
	BitOrAssign,
	#[token("^=")]
	BitXorAssign,
	#[token("&=")]
	BitAndAssign,
	#[token("**=")]
	ExpAssign,

	#[token("++")]
	PlusPlus,
	#[token("--")]
	MinusMinus,

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

	#[token("!")]
	Exclamation,
	#[token("~")]
	Tilde,
	#[token("'")]
	Tag,
	#[token("\"")]
	TagDouble,
	#[token("`")]
	BackTick,
	#[token("@")]
	At,
	#[token("...")]
	Spread,
	#[token("=>")]
	ArrowFn,
	#[token("?.")]
	OptionalChaining,

	#[token("let ")]
	Let,
	#[token("const ")]
	Const,
	#[token("this")]
	This,
	#[token("globalThis")]
	GlobalThis,
	#[token("process")]
	Process,
	#[token("window")]
	Window,
	#[token("console")]
	Console,
	#[token("function")]
	Function,
	#[token("void")]
	Void,
	#[token("delete")]
	Delete,
	#[token("null")]
	Null,
	#[token("undefined")]
	Undefined,
	#[token("in")]
	In,
	#[token("typeof")]
	TypeOf,
	#[token("instanceof")]
	InstanceOf,
	#[token("enum")]
	Enum,
	#[token("class")]
	Class,
	#[token("interface")]
	Interface,

	#[regex(r#"<[a-zA-Z\s_='"-]+></[a-zA-Z-]+\s?/>"#, |lex| lex.slice().parse())]
	HtmlTag(String),
	#[regex(r#"<[a-zA-Z\s_'"-]+ />"#, |lex| lex.slice().parse())]
	HtmlTagSelfClosing(String),

	#[token(".")]
	Period,
	#[token(",")]
	Comma,
	#[token(";")]
	SemiColon,
	#[token(":")]
	Colon,

	// TODO: fix import path not found
	#[regex(r"'([^']*)'", get_string_content)]
	StringSingle(String),
	#[regex(r#""[^"]*""#, get_string_content)]
	StringDouble(String),
	#[regex(r#"`(?s)[^`]*(?-s)`"#, get_string_content)]
	StringTemplate(String),
	#[regex(r#"/.+/"#, get_string_content)]
	JsRegex(String),
	#[regex("[a-zA-Z0-9$_]+", |lex| lex.slice().parse())]
	Text(String),

	// TODO: fix this... (does not find '12e3')
	#[regex(r"-?\d+(?:e\d+)?", |lex| lex.slice().parse(), priority = 2)]
	Int(i32),
	#[regex(r"-?\d+\.\d*(?:e\d+)?", |lex| lex.slice().parse())]
	Float(f64),

	#[regex(r"[\s\t\n\f]+", logos::skip)]
	#[error]
	Error,
}

#[cfg(test)]
mod token_tests {
	use super::*;

	#[test]
	fn sanity() {
		let mut lex = Token::lexer(r"Some test string.");

		assert_eq!(lex.next(), Some(Token::Text("Some".to_string())));
		assert_eq!(lex.span(), 0..4);
		assert_eq!(lex.slice(), "Some");

		assert_eq!(lex.next(), Some(Token::Text("test".to_string())));
		assert_eq!(lex.span(), 5..9);
		assert_eq!(lex.slice(), "test");

		assert_eq!(lex.next(), Some(Token::Text("string".to_string())));
		assert_eq!(lex.span(), 10..16);
		assert_eq!(lex.slice(), "string");

		assert_eq!(lex.next(), Some(Token::Period));
		assert_eq!(lex.span(), 16..17);
		assert_eq!(lex.slice(), ".");

		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_console_log() {
		let mut lex = Token::lexer(r"console.log(123);");

		assert_eq!(lex.next(), Some(Token::Console));
		assert_eq!(lex.next(), Some(Token::Period));
		assert_eq!(lex.next(), Some(Token::Text("log".to_owned())));
		assert_eq!(lex.next(), Some(Token::BracketOpen));
		assert_eq!(lex.next(), Some(Token::Int(123)));
		assert_eq!(lex.next(), Some(Token::BracketClose));
	}

	#[test]
	fn parse_number_basic_int() {
		let mut lex = Token::lexer(r"12");

		assert_eq!(lex.next(), Some(Token::Int(12)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_neg_int() {
		let mut lex = Token::lexer(r"-12");

		assert_eq!(lex.next(), Some(Token::Int(-12)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_basic_float() {
		let mut lex = Token::lexer(r"12.");

		assert_eq!(lex.next(), Some(Token::Float(12.0)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_neg_float() {
		let mut lex = Token::lexer(r"-12.");

		assert_eq!(lex.next(), Some(Token::Float(-12.0)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	#[ignore]
	fn parse_number_exp_int() {
		let mut lex = Token::lexer(r"12e3");

		assert_eq!(lex.next(), Some(Token::Int(12_000)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_exp_float() {
		let mut lex = Token::lexer(r"13.e2");

		assert_eq!(lex.next(), Some(Token::Float(13e2)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_float() {
		let mut lex = Token::lexer(r"123123.");

		assert_eq!(lex.next(), Some(Token::Float(123_123.0)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn parse_number_float_2() {
		let mut lex = Token::lexer(r"123123.55");

		assert_eq!(lex.next(), Some(Token::Float(123_123.55)));
		assert_eq!(lex.next(), None);
	}

	#[test]
	#[ignore]
	fn parse_expr() {
		let mut lex = Token::lexer(r#"23e3 !== 22."#);

		assert_eq!(lex.next(), Some(Token::Int(23_000)), "{}", lex.slice());
		assert_eq!(lex.next(), Some(Token::NotEqEq));
		assert_eq!(lex.next(), Some(Token::Float(22.0)));
		assert_eq!(lex.next(), None);
	}
}
