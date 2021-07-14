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

    for ch in src.chars() {
        let tok = match ch {
            ' ' => continue,
            '+' | '-' => Token { kind: TokenKind::Reserved, val: None, ch: Some(ch) },
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => Token { kind: TokenKind::Num, val: Some(char_to_num(ch)), ch: Some(ch) },
            _ => panic!("invalid token"),
        };

        tokens.push(tok);
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

