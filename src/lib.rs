pub mod token;
pub mod parse;

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
    Else,           // else
    While,          // while
    For,            // for
    Eq,             // ==
    Ne,             // !=
    Le,             // <=
    Ge,             // >=
    Lt,             // <
    Gt,             // >
    Assign,         // =
    Semicolon,      // ;
    Comma,           // ,
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
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
            ',' => Some(Comma),
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            '{' => Some(LeftBrace),
            '}' => Some(RightBrace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CharacterType {
    Whitespace,     // ' '
    NewLine,        // \n
    CarriageReturn, // \r
    Alphabetic,
    Digit,
    NonAlphabetic(char),
    Unknown(char),
}