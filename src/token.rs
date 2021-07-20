use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum TokenKind {
    Reserved,
    Num,
    Eof
}

pub struct Token {
    pub kind: TokenKind,
    pub val: Option<i32>,
    pub ch: Option<char>,
    pub len: i32
}

pub fn new_token(kind: TokenKind, val: Option<i32>, s: Option<char>, len: i32) -> Token {
    return Token {
        kind: kind,
        val: val,
        ch: s,
        len: len
    };
}

pub fn tokenize(src: &str) -> VecDeque<Token> {
    let cs: Vec<char> = src.chars().collect();
    let mut chars = VecDeque::from(cs);

    let mut tokens: VecDeque<Token> = VecDeque::new();

    loop {
        let ch = match chars.front() {
            Some(c) => c,
            None => break,
        };

        if *ch == ' ' {
            chars.pop_front();
            continue;
        }

        if *ch == '+' || *ch == '-' || *ch == '*' || *ch == '/' || *ch == '(' || *ch == ')' {
            let c = chars.pop_front().unwrap();
            let token = new_token(TokenKind::Reserved, None, Some(c), 0);
            tokens.push_back(token);
            continue;
        }
        
        if is_num(*ch) {
            let (que, num, len) = lookahead_for_num(chars);
            chars = que;
            let token = new_token(TokenKind::Num, Some(num), None, len);
            tokens.push_back(token);
            continue;
        }

        panic!("not support character");
    }

    return tokens;
}

pub fn lookahead_for_num(chars: VecDeque<char>) -> (VecDeque<char>, i32, i32) {
    let mut chars = chars;
    let mut buf = String::new();

    while let Some(c) = chars.front() {
        if !is_num(*c) {
            break;
        }
        let ch = chars.pop_front().unwrap();
        buf.push(ch);
    }

    let len = buf.len() as i32;
    let num = buf.parse::<i32>().unwrap();
    return (chars, num, len);
}

pub fn char_to_num(ch: char) -> i32 {
    return ch as i32 - 48;
}

pub fn is_num(ch: char) -> bool {
    let num = char_to_num(ch);
    return 0 <= num && num <= 9;
}

