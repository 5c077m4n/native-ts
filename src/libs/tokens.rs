use logos::Logos;

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
    #[token("in")]
    In,
    #[token("instanceo")]
    InstanceOf,
    #[token("**")]
    Exp,
    #[token("??")]
    NullishCoalescing,

    #[token(".")]
    Period,
    #[regex("[a-zA-Z]+")]
    String,
    #[error]
    Error,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[ignore]
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
}
