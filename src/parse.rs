use std::collections::VecDeque;

use super::token::*;

#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Box<Option<Node>>,
    pub rhs: Box<Option<Node>>,
    pub val: isize,
}

// expr = mul ("+" mul | "-" mul)*
pub fn expr(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let mut token_que = VecDeque::from(tokens);

    let ret = mul(token_que);
    let mut node = ret.0;
    token_que = ret.1;

    loop {
        let token = match token_que.front() {
            Some(tk) => tk,
            None => break,
        };
        if token.kind == TokenKind::Eof {
            break;
        }

        if token.kind == TokenKind::Reserved {
            if token.ch.unwrap() == '+' {
                token_que.pop_front();
                let _lhs = Box::new(Some(node));
                let ret = mul(token_que);
                let _rhs = Box::new(Some(ret.0));
                token_que = ret.1;

                node = Node {
                    kind: NodeKind::Add,
                    lhs: _lhs,
                    rhs: _rhs,
                    val: 0,
                };
                continue;
            }
            else if token.ch.unwrap() == '-' {
                token_que.pop_front();
                let _lhs = Box::new(Some(node));
                let ret = mul(token_que);
                let _rhs = Box::new(Some(ret.0));
                token_que = ret.1;

                node = Node {
                    kind: NodeKind::Sub,
                    lhs: _lhs,
                    rhs: _rhs,
                    val: 0,
                };
                continue;
            }
        }
        break;
    }

    return (node, token_que);
}
// mul = primary ("*" primary | "/" primary)*
pub fn mul(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let (mut node, mut que) = primary(tokens);

    loop {
        let token = match que.front() {
            Some(tk) => tk,
            None => break,
        };

        if token.kind == TokenKind::Reserved {
            if token.ch.unwrap() == '*' {
                que.pop_front();
                let _lhs = Box::new(Some(node));
                let ret = primary(que);
                let _rhs = Box::new(Some(ret.0));
                que = ret.1;
    
                node = Node {
                    kind: NodeKind::Mul,
                    lhs: _lhs,
                    rhs: _rhs,
                    val: 0,
                };
                continue;
            }
            else if token.ch.unwrap() == '/' {
                que.pop_front();
                let _lhs = Box::new(Some(node));
                let ret = primary(que);
                let _rhs = Box::new(Some(ret.0));
                que = ret.1;
    
                node = Node {
                    kind: NodeKind::Div,
                    lhs: _lhs,
                    rhs: _rhs,
                    val: 0,
                };
                continue;
            }
        }

        break;
    }

    return (node, que);
}

// primary = num | "(" expr ")"
pub fn primary(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let mut que = VecDeque::from(tokens);
    let token = que.pop_front().unwrap();
    if token.kind == TokenKind::Reserved {
        if token.ch.unwrap() == '(' {
            let (node, mut q) = expr(que);
            q.pop_front();
            return (node, q);
        }
        panic!("not support operation");
    }

    let node = Node {
        kind: NodeKind::Num,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: token.val.unwrap() as isize
    };
    return (node, que);
}