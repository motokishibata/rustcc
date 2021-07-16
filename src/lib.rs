pub mod compile;
pub mod token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_test1() {
        let tokens = token::tokenize("1 +2");
        let tok = &tokens[0];
        assert_eq!(Some(1), tok.val);
    }

    #[test]
    fn tokenize_test2() {
        let tokens = token::tokenize("1 +2");
        let tok = &tokens[1];
        assert_eq!(Some('+'), tok.ch);
    }

    #[test]
    fn tokenize_test3() {
        let tokens = token::tokenize("1 +2");
        let tok = &tokens[tokens.len() - 1];
        let _k = &tok.kind;
        assert!(matches!(token::TokenKind::Eof, _k));
    }

    #[test]
    fn tokenize_test4() {
        let tokens = token::tokenize("10+2");
        let tok = &tokens[0];
        assert_eq!(Some(10), tok.val);
        let tok2 = &tokens[1];
        assert_eq!(Some('+'), tok2.ch);
        let tok3 = &tokens[2];
        assert_eq!(Some(2), tok3.val);
    }
}