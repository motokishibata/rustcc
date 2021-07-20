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

fn new_node(kind: NodeKind, lhs: Option<Node>, rhs: Option<Node>) -> Node {
    return Node {
        kind: kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        val: 0
    };
}

fn new_num_node(val: isize) -> Node {
    return Node {
        kind: NodeKind::Num,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: val,
    };
}

// expr = mul ("+" mul | "-" mul)*
pub fn expr(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let mut tokens = tokens;

    let ret = mul(tokens);
    let mut node = ret.0;
    tokens = ret.1;

    loop {
        let token = match tokens.front() {
            Some(tk) => tk,
            None => break,
        };
        if token.kind == TokenKind::Eof {
            break;
        }

        if token.kind == TokenKind::Reserved {
            if token.get_string() == "+" {
                tokens.pop_front();

                let ret = mul(tokens);
                tokens = ret.1;

                node = new_node(NodeKind::Add, Some(node), Some(ret.0));
                continue;
            }
            else if token.get_string() == "-" {
                tokens.pop_front();

                let ret = mul(tokens);
                tokens = ret.1;

                node = new_node(NodeKind::Sub, Some(node), Some(ret.0));
                continue;
            }
        }
        break;
    }

    return (node, tokens);
}
// mul = unary ("*" unary | "/" unary)*
pub fn mul(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let (mut node, mut tokens) = unary(tokens);

    loop {
        let token = match tokens.front() {
            Some(tk) => tk,
            None => break,
        };

        if token.kind == TokenKind::Reserved {
            if token.get_string() == "*" {
                tokens.pop_front();
                let ret = unary(tokens);
                tokens = ret.1;

                node = new_node(NodeKind::Mul, Some(node), Some(ret.0));
                continue;
            }
            else if token.get_string() == "/" {
                tokens.pop_front();

                let ret = unary(tokens);
                tokens = ret.1;
    
                node = new_node(NodeKind::Div, Some(node), Some(ret.0));
                continue;
            }
        }

        break;
    }

    return (node, tokens);
}

// unary = ("+" | "-")? primary
pub fn unary(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let mut tokens = tokens;
    let token = tokens.front().unwrap();

    if token.kind == TokenKind::Reserved {
        if token.get_string() == "+" {
            tokens.pop_front();
            return primary(tokens);
        }
        else if token.get_string() == "-" {
            tokens.pop_front();
            let _lhs = new_num_node(0);

            let ret = primary(tokens);
            tokens = ret.1;

            let node = new_node(NodeKind::Sub, Some(_lhs), Some(ret.0));
            return (node, tokens);
        }
    }

    return primary(tokens);
}

// primary = num | "(" expr ")"
pub fn primary(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let mut que = VecDeque::from(tokens);
    let token = que.pop_front().unwrap();
    if token.kind == TokenKind::Reserved {
        if token.st.unwrap() == "(" {
            let (node, mut q) = expr(que);
            q.pop_front();
            return (node, q);
        }
        panic!("not support operation");
    }
    
    let node = new_num_node(token.val.unwrap() as isize);
    return (node, que);
}