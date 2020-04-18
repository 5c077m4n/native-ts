use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Uni operators
    #[token = "!"]
    Not,
    // Binary operators
    #[token = "."]
    Period,
    #[token = "=="]
    Eq,
    #[token = "!="]
    Ne,
    #[token = "==="]
    TripleEq,
    #[token = "!=="]
    TripleNe,
    // KeyWords
    #[token = "this"]
    This,
    // General
    #[error]
    Error,
    #[regex = "[a-zA-Z]+"]
    String,
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
}
