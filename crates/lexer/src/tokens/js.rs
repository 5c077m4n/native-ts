use logos::{Lexer, Logos};

fn get_string_content(lex: &mut Lexer<JsToken>) -> Option<String> {
	let lex = lex.slice();
	let content: String = lex[1..(lex.len() - 1)].parse().ok()?;

	Some(content)
}

#[derive(Logos, Debug, PartialEq)]
pub enum JsToken {
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

	#[regex(r#"-?\d+(\.\d*)?(?:e\d+)?"#, |lex| lex.slice().parse(), priority = 2)]
	Number(f64),

	#[regex(r"[\s\t\n\f]+", logos::skip)]
	#[error]
	Error,
}
