use logos::{self, Logos};

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

    #[token("let")]
    Let,
    #[token("const")]
    Const,

    #[token("!")]
    Exclamation,
    #[token("~")]
    Tilde,
    #[token("'")]
    Tag,
    #[token("\"")]
    DoubleTag,
    #[token("`")]
    SideTag,
    #[token("@")]
    At,

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

    #[regex(r"<[a-zA-Z\s_=-]+></[a-zA-Z-]+ />")]
    HtmlTag,
    #[regex(r"<[a-zA-Z\s_-]+ />")]
    SelfClosingHtmlTag,

    #[token(".")]
    Period,
    #[token(";")]
    SemiColon,
    #[token(":")]
    Colon,
    #[regex("[a-zA-Z]+")]
    String,
    #[regex(r"\d+(?:e\d+)?")]
    Int,
    #[regex(r"\d+\.\d*(?:e\d+)?")]
    Float,

    #[regex(r"[\s\t\n\f]+", logos::skip)]
    Skip,
    #[error]
    Error,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let mut lex = Token::lexer("Some test string.");

        assert_eq!(lex.next(), Some(Token::String));
        assert_eq!(lex.span(), 0..4);
        assert_eq!(lex.slice(), "Some");

        assert_eq!(lex.next(), Some(Token::String));
        assert_eq!(lex.span(), 5..9);
        assert_eq!(lex.slice(), "test");

        assert_eq!(lex.next(), Some(Token::String));
        assert_eq!(lex.span(), 10..16);
        assert_eq!(lex.slice(), "string");

        assert_eq!(lex.next(), Some(Token::Period));
        assert_eq!(lex.span(), 16..17);
        assert_eq!(lex.slice(), ".");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn parse_number_basic() {
        let mut lex = Token::lexer("1");

        assert_eq!(lex.next(), Some(Token::Int));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn parse_number_exp() {
        let mut lex = Token::lexer("123e12");

        assert_eq!(lex.next(), Some(Token::Int));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn parse_number_frac() {
        let mut lex = Token::lexer("123123.");

        assert_eq!(lex.next(), Some(Token::Float));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn parse_number_frac_2() {
        let mut lex = Token::lexer("123123.55");

        assert_eq!(lex.next(), Some(Token::Float));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn parse_expr() {
        let mut lex = Token::lexer("23e25 !== 22.");

        assert_eq!(lex.next(), Some(Token::Int));
        assert_eq!(lex.next(), Some(Token::NotEqEq));
        assert_eq!(lex.next(), Some(Token::Float));
        assert_eq!(lex.next(), None);
    }
}
