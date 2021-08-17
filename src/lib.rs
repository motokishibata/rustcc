pub mod token;
pub mod parse;

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
        assert_eq!(Some("+"), tok.st);
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
        assert_eq!(Some("+"), tok2.st);
        let tok3 = &tokens[2];
        assert_eq!(Some(2), tok3.val);
    }

    // #[test]
    // fn parse_test() {
    //     let tokens = token::tokenize("1+2");
    //     let (node, _) = parse::expr(VecDeque::from(tokens));

    //     let _n = &node.kind;
    //     assert!(matches!(parse::NodeKind::Add, _n));
    //     let left = (*node.lhs).unwrap();
    //     assert_eq!(1, left.val);
    //     let right = (*node.rhs).unwrap();
    //     assert_eq!(2, right.val);
    // }

    // #[test]
    // fn parse_test2() {
    //     let tokens = token::tokenize("1*2");
    //     let (node, _) = parse::expr(VecDeque::from(tokens));

    //     let _n = &node.kind;
    //     assert!(matches!(parse::NodeKind::Mul, _n));
    //     let left: parse::Node = (*node.lhs).unwrap();
    //     assert_eq!(1, left.val);
    //     let right: parse::Node = (*node.rhs).unwrap();
    //     assert_eq!(2, right.val);
    // }

    // #[test]
    // fn parse_test3() {
    //     let tokens = token::tokenize("1+(5-3)");
    //     let (node, _) = parse::expr(VecDeque::from(tokens));

    //     let _n = &node.kind;
    //     assert!(matches!(parse::NodeKind::Add, _n));
    //     let left: parse::Node = (*node.lhs).unwrap();
    //     assert_eq!(1, left.val);
    //     let right: parse::Node = (*node.rhs).unwrap();
    //     let _n2 = &right.kind;
    //     assert!(matches!(parse::NodeKind::Sub, _n2));
    // }
}