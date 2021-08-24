mod token2;
pub mod parse2;

#[cfg(test)]
mod tokenize_test {
    use crate::TokenType;
    use crate::token2::*;

    #[test]
    fn it_single_digit() {
        let tokens = tokenize("0");
        assert_eq!(tokens, vec![
            TokenType::Num(0),
        ]);
    }

    #[test]
    fn it_calc_add() {
        let tokens = tokenize("0+1");
        assert_eq!(tokens, vec![
            TokenType::Num(0),
            TokenType::Plus,
            TokenType::Num(1),
        ]);
    }

    #[test]
    fn it_var() {
        let tokens = tokenize("abc = 1");
        assert_eq!(tokens, vec![
            TokenType::Ident("abc".into()),
            TokenType::Assign,
            TokenType::Num(1),
        ]);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Num(i32),       // 数値
    Ident(String),  // 識別子(変数名)
    Return,         // return
    Plus,           // +
    Minus,          // -
    Mul,            // *
    Div,            // /
    If,             // if
    Eq,             // ==
    Ne,             // !=
    Le,             // <=
    Ge,             // >=
    Lt,             // <
    Gt,             // >
    Assign,         // =
    Semicolon,      // ;
    LeftParen,      // (
    RightParen,     // )
    NewLine,        // 改行
}

impl TokenType {
    pub fn new_single_letter(c: char) -> Option<Self> {
        use self::TokenType::*;
        match c {
            '+' => Some(Plus),
            '-' => Some(Minus),
            '*' => Some(Mul),
            '/' => Some(Div),
            '=' => Some(Assign),
            '<' => Some(Lt),
            '>' => Some(Gt),
            ';' => Some(Semicolon),
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CharacterType {
    Whitespace,     // ' '
    NewLine,        // \n
    Alphabetic,
    Digit,
    NonAlphabetic(char),
    Unknown(char),
}