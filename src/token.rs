use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::TokenType;
use crate::CharacterType;

pub fn tokenize(src: &str) -> Vec<TokenType> {
    let mut tokenizer = Tokenizer::new(src.chars().collect());
    tokenizer.scan(&keyword_map());
    tokenizer.tokens
}

fn keyword_map() -> HashMap<String, TokenType> {
    let mut map = HashMap::new();
    map.insert("if".into(), TokenType::If);
    map.insert("else".into(), TokenType::Else);
    map.insert("return".into(), TokenType::Return);
    map
}

#[derive(Debug, Clone)]
struct Symbol {
    name: &'static str,
    ty: TokenType,
}

impl Symbol {
    fn new(name: &'static str, ty: TokenType) -> Self {
        Symbol { name, ty }
    }
}

static SYMBOLS: Lazy<Vec<Symbol>> = Lazy::new(|| [
    Symbol::new("==", TokenType::Eq),
    Symbol::new("!=", TokenType::Ne),
    Symbol::new("<=", TokenType::Le),
    Symbol::new(">=", TokenType::Ge),
].to_vec());

struct Tokenizer {
    src: Vec<char>,
    pos: usize,
    tokens: Vec<TokenType>
}

impl Tokenizer {
    fn new(src: Vec<char>) -> Self {
        Tokenizer {
            src,
            pos: 0,
            tokens: Vec::from(vec![])
        }
    }

    fn get_character(&self, advance_from_pos: usize) -> Option<CharacterType> {
        self.src.get(self.pos + advance_from_pos).map(|ch| {
            if ch == &'\n' {
                CharacterType::NewLine
            } else if ch == &' ' || ch == &'\t' {
                CharacterType::Whitespace
            } else if ch.is_alphabetic() || ch == &'_' {
                CharacterType::Alphabetic
            } else if ch.is_ascii_digit() {
                CharacterType::Digit
            } else {
                CharacterType::NonAlphabetic(*ch)
            }
        })
    }

    fn scan(&mut self, keywords: &HashMap<String, TokenType>) -> Vec<TokenType> {
        'outer: while let Some(head_char) = self.get_character(0) {
            match head_char {
                CharacterType::NewLine => {
                    self.pos += 1;
                    self.tokens.push(TokenType::NewLine)
                },
                CharacterType::Whitespace => self.pos += 1,
                CharacterType::Alphabetic => self.ident(&keywords),
                CharacterType::Digit => self.number(),
                CharacterType::NonAlphabetic(c) => {
                    // Multi-letter symbol
                    for symbol in SYMBOLS.iter() {
                        let name = symbol.name;
                        let len = name.len();
                        if self.pos + len > self.src.len() {
                            continue;
                        }

                        let first = &self.src[self.pos..(self.pos + len)];
                        if name != first.iter().collect::<String>() {
                            continue;
                        }

                        self.pos += len;
                        self.tokens.push(symbol.ty.clone());
                        continue 'outer;
                    }

                    // Single-letter symbol
                    if let Some(ty) = TokenType::new_single_letter(c) {
                        self.pos += 1;
                        self.tokens.push(ty);
                        continue 'outer;
                    }
                    panic!("not support symbol");
                },
                CharacterType::Unknown(_) => panic!("not support character")
            }
        }
        self.tokens.clone()
    }

    fn ident(&mut self, keywords: &HashMap<String, TokenType>) {
        let mut len = 1;
        while let Some(c2) = self.src.get(self.pos + len) {
            if c2.is_alphabetic() || c2.is_ascii_digit() || c2 == &'_' {
                len += 1;
                continue;
            }
            break;
        }

        let name: String = self.src[self.pos..(self.pos + len)].iter().collect();
        let t;
        if let Some(keyword) = keywords.get(&name) {
            t = keyword.clone();
        } else {
            t = TokenType::Ident(name.clone());
        }

        self.pos += len;
        self.tokens.push(t);
    }

    fn number(&mut self) {
        let mut sum: i32 = 0;
        let mut len = 0;
        for c in self.src[self.pos..].iter() {
            if let Some(val) = c.to_digit(10) {
                sum = sum * 10 as i32 + val as i32;
                len += 1;
            } else {
                break;
            }
        }
        self.pos += len;
        self.tokens.push(TokenType::Num(sum as i32));
    }
}