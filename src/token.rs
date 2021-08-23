use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Reserved,
    Ident,
    Num,
    Return,
    Eof
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub val: Option<i32>,
    pub st: Option<String>,
    pub len: i32
}

impl Token {
    pub fn get_string(&self) -> &str {
        match &self.st {
            Some(s) => return s,
            None => return "",
        };
    }

    pub fn new(kind: TokenKind, val: Option<i32>, st: Option<String>, len: i32) -> Token {
        return Token {
            kind: kind,
            val: val,
            st: st,
            len: len
        };
    }
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

        if is_reserved(*ch) {
            let (que, st, len) = lookahead_for_reserved(chars);
            chars = que;
            let token = Token::new(TokenKind::Reserved, None, Some(st), len);
            tokens.push_back(token);
            continue;
        }
        
        if ch.is_numeric() {
            let (que, num, len) = lookahead_for_num(chars);
            chars = que;
            let token = Token::new(TokenKind::Num, Some(num), None, len);
            tokens.push_back(token);
            continue;
        }

        if is_return(&chars) {
            for _ in 0..6 {
                chars.pop_front();
            }
            let token = Token::new(TokenKind::Return, None, None, 6);
            tokens.push_back(token);
            continue;
        }

        if ch.is_ascii_alphabetic() {
            let (que, st, len) = lookahead_for_ident(chars);
            chars = que;
            let token = Token::new(TokenKind::Ident, None, Some(st), len);
            tokens.push_back(token);
            continue;
        }

        panic!("not support character");
    }

    let eof = Token::new(TokenKind::Eof, None, None, 0);
    tokens.push_back(eof);

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

pub fn lookahead_for_reserved(chars: VecDeque<char>) -> (VecDeque<char>, String, i32) {
    let mut chars = chars;
    let mut buf = String::new();

    if let Some(c) = chars.front() {
        if "+-*/();".contains(*c) {
            let ch = chars.pop_front().unwrap();
            buf.push(ch);
        }
        else if "=!<>".contains(*c) {
            let ch = chars.pop_front().unwrap();
            buf.push(ch);

            if let Some(c) = chars.front() {
                if '=' == *c {
                    let ch = chars.pop_front().unwrap();
                    buf.push(ch);
                }
            }
        }
    }

    let len = buf.len() as i32;
    return (chars, buf, len);
}

pub fn is_reserved(ch: char) -> bool {
    return "+-*/()=!<>;".contains(ch);
}

pub fn lookahead_for_ident(chars: VecDeque<char>) -> (VecDeque<char>, String, i32) {
    let mut chars = chars;
    let mut buf = String::new();

    while let Some(c) = chars.front() {
        if !c.is_ascii_alphabetic() && !(*c == '_') {
            break;
        }
        let ch = chars.pop_front().unwrap();
        buf.push(ch);
    }

    let len = buf.len() as i32;
    return (chars, buf, len);
}

fn is_return(chars: &VecDeque<char>) -> bool {
    if chars.len() < 6 {
        return false;
    }

    let mut r = VecDeque::from(vec!['r','e','t','u','r','n']);
    
    for ch in chars {
        if r.is_empty() {
            return !ch.is_ascii_alphanumeric() && *ch != '_';
        }

        if let Some(c) = r.pop_front() {
            if c != *ch {
                return false;
            }
        }
    }

    return false;
}