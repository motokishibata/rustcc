pub mod compile;
pub mod token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_number_test() {
        let ret = compile::len_number("1+1");
        assert_eq!(ret, 1);
    }
    #[test]
    fn len_number_test2() {
        let ret = compile::len_number("10-");
        assert_eq!(ret, 2);
    }

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
        let k = &tok.kind;
        assert!(matches!(token::TokenKind::Eof, k));
    }
}