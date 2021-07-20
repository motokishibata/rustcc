use std::collections::VecDeque;

use super::token::*;

#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    Eq,     // ==
    Ne,     // !=
    Le,     // <=
    Lt,     // <
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

// expr = equality
pub fn expr(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    return equality(tokens);
}

// equality = relational ("==" relational | "!=" relational)*
pub fn equality(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let (mut node, mut tokens) = relational(tokens);

    loop {
        let token = match tokens.front() {
            Some(tk) => {
                if tk.kind == TokenKind::Eof || tk.kind != TokenKind::Reserved {
                    break;
                }
                tk
            },
            None => break,
        };

        let nodekind = match token.get_string() {
            "==" => NodeKind::Eq,
            "!=" => NodeKind::Ne,
            _ => break,
        };

        tokens.pop_front();
        let ret = relational(tokens);
        tokens = ret.1;

        node = new_node(nodekind, Some(node), Some(ret.0));
    }

    return (node, tokens);
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
pub fn relational(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let (mut node, mut tokens) = add(tokens);

    loop {
        let token = match tokens.front() {
            Some(tk) => {
                if tk.kind != TokenKind::Reserved {
                    break;
                }
                tk
            },
            None => break,
        };

        let (nodekind, switch) = match token.get_string() {
            "<" => (NodeKind::Lt, false),
            "<=" => (NodeKind::Le, false),
            ">" => (NodeKind::Lt, true),
            ">=" => (NodeKind::Le, true),
            _ => break,
        };

        tokens.pop_front();
        let ret = add(tokens);
        tokens = ret.1;

        if switch {
            node = new_node(nodekind, Some(ret.0), Some(node));
        } else {
            node = new_node(nodekind, Some(node), Some(ret.0));
        }
    }

    return (node, tokens);
}

// add = mul ("+" mul | "-" mul)*
pub fn add(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let (mut node, mut tokens) = mul(tokens);

    loop {
        let token = match tokens.front() {
            Some(tk) => {
                if tk.kind != TokenKind::Reserved {
                    break;
                }
                tk
            },
            None => break,
        };

        let nodekind = match token.get_string() {
            "+" => NodeKind::Add,
            "-" => NodeKind::Sub,
            _ => break,
        };

        tokens.pop_front();
        let ret = mul(tokens);
        tokens = ret.1;

        node = new_node(nodekind, Some(node), Some(ret.0));
    }

    return (node, tokens);
}

// mul = unary ("*" unary | "/" unary)*
pub fn mul(tokens: VecDeque<Token>) -> (Node, VecDeque<Token>) {
    let (mut node, mut tokens) = unary(tokens);

    loop {
        let token = match tokens.front() {
            Some(tk) => {
                if tk.kind != TokenKind::Reserved {
                    break;
                }
                tk
            },
            None => break,
        };

        let nodekind = match token.get_string() {
            "*" => NodeKind::Mul,
            "/" => NodeKind::Div,
            _ => break,
        };

        tokens.pop_front();
        let ret = unary(tokens);
        tokens = ret.1;

        node = new_node(nodekind, Some(node), Some(ret.0));
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
    let mut tokens = tokens;
    let token = tokens.pop_front().unwrap();

    if token.kind == TokenKind::Reserved {
        if token.get_string() == "(" {
            let (node, mut q) = expr(tokens);
            q.pop_front();
            return (node, q);
        }
        panic!("not support operation");
    }
    
    let node = new_num_node(token.val.unwrap() as isize);
    return (node, tokens);
}