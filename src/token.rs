#[derive(PartialEq)]
pub enum TokenKind {
    Reserved,
    Num,
    Eof
}

pub struct Token {
    pub kind: TokenKind,
    pub val: Option<i32>,
    pub ch: Option<char>
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut count = 0;
    while count < src.chars().count() {
        let c = src.chars().nth(count).unwrap();

        if c == ' ' {
            count += 1;
            continue;
        }

        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            tokens.push(Token { kind: TokenKind::Reserved, val: None, ch: Some(c) });
            count += 1;
            continue;
        }

        if is_num(c) {
            let mut num_buf = String::new();
            num_buf.push(c);
            count += 1;
            for ch in src.chars().skip(count) {
                if is_num(ch) {
                    num_buf.push(ch);
                    count += 1;
                } else {
                    break;
                }
            }
            let num: i32 = num_buf.parse().unwrap();
            tokens.push(Token { kind: TokenKind::Num, val: Some(num), ch: None });
            continue;
        }

        panic!("not support character");
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        val: None,
        ch: None,
    });
    return tokens;
}

pub fn char_to_num(ch: char) -> i32 {
    return ch as i32 - 48;
}

pub fn is_num(ch: char) -> bool {
    let num = char_to_num(ch);
    return 0 <= num && num <= 9;
}

