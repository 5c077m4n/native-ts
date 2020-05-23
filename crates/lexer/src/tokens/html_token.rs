use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum HtmlToken {
	#[regex(r#"<[a-zA-Z\s_='"-]+></[a-zA-Z-]+\s?/>"#, |lex| lex.slice().parse())]
	HtmlTag(String),
	#[regex(r#"<[a-zA-Z\s_'"-]+ />"#, |lex| lex.slice().parse())]
	HtmlTagSelfClosing(String),

	#[regex(r"[\s\t\n\f]+", logos::skip)]
	#[error]
	Error,
}
