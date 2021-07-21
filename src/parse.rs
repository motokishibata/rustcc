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
    Assign, // =
    LVar,   // ローカル変数
    Return, // return
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Box<Option<Node>>,
    pub rhs: Box<Option<Node>>,
    pub val: isize,
    pub offset: isize,
}

fn new_node(kind: NodeKind, lhs: Option<Node>, rhs: Option<Node>) -> Node {
    return Node {
        kind: kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        val: 0,
        offset: 0,
    };
}

fn new_num_node(val: isize) -> Node {
    return Node {
        kind: NodeKind::Num,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: val,
        offset: 0,
    };
}

pub struct LVar {
    pub name: String,
    pub len: i32,
    pub offset: i32,
}

pub fn find_lvar(tok: Token, locals: &Vec<LVar>) -> Option<&LVar> {
    for var in locals {
        if var.len == tok.len && var.name == tok.get_string() {
            return Some(var);
        }
    }
    return None;
}

// program = stmt*
pub fn program(tokens: VecDeque<Token>) -> Vec<Node> {
    let mut code: Vec<Node> = Vec::new();
    let locals: Vec<LVar> = Vec::new();
    let (mut node, mut tokens, mut locals) = stmt(tokens, locals);
    code.push(node);

    loop {
        match tokens.front() {
            Some(tk) => {
                if tk.kind == TokenKind::Eof {
                    break;
                }
            },
            None => panic!("not found eof token"),
        }

        let ret = stmt(tokens, locals);
        node = ret.0;
        tokens = ret.1;
        locals = ret.2;
        code.push(node);
    }

    return code;
}

// stmt = expr ";" | "return" expr ";"
pub fn stmt(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let node: Node;
    let mut tokens = tokens;
    let mut locals = locals;
    let token = tokens.front().unwrap();

    if token.kind == TokenKind::Return {
        tokens.pop_front();
        let ret = expr(tokens, locals);
        node = new_node(NodeKind::Return, Some(ret.0), None);
        tokens = ret.1;
        locals = ret.2;
    } else {
        let ret = expr(tokens, locals);
        node = ret.0;
        tokens = ret.1;
        locals = ret.2;
    }

    let token = tokens.pop_front().unwrap();
    if token.st != Some(";".to_string()) {
        panic!("require ;");
    }

    return (node, tokens, locals);
}

// expr = equality
pub fn expr(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    return assign(tokens, locals);
}

// assign = equality ("=" assign)?
pub fn assign(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let (mut node, mut tokens, mut locals) = equality(tokens, locals);

    if let Some(tk) = tokens.front() {
        if tk.get_string() == "=" {
            tokens.pop_front();
            let ret = assign(tokens, locals);
            tokens = ret.1;
            locals = ret.2;

            node = new_node(NodeKind::Assign, Some(node), Some(ret.0));
        }
    }

    return (node, tokens, locals);
}

// equality = relational ("==" relational | "!=" relational)*
pub fn equality(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let (mut node, mut tokens, mut locals) = relational(tokens, locals);

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
        let ret = relational(tokens, locals);
        tokens = ret.1;
        locals = ret.2;

        node = new_node(nodekind, Some(node), Some(ret.0));
    }

    return (node, tokens, locals);
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
pub fn relational(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let (mut node, mut tokens, mut locals) = add(tokens, locals);

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
        let ret = add(tokens, locals);
        tokens = ret.1;
        locals = ret.2;

        if switch {
            node = new_node(nodekind, Some(ret.0), Some(node));
        } else {
            node = new_node(nodekind, Some(node), Some(ret.0));
        }
    }

    return (node, tokens, locals);
}

// add = mul ("+" mul | "-" mul)*
pub fn add(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let (mut node, mut tokens, mut locals) = mul(tokens, locals);

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
        let ret = mul(tokens, locals);
        tokens = ret.1;
        locals = ret.2;

        node = new_node(nodekind, Some(node), Some(ret.0));
    }

    return (node, tokens, locals);
}

// mul = unary ("*" unary | "/" unary)*
pub fn mul(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let (mut node, mut tokens, mut locals) = unary(tokens, locals);

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
        let ret = unary(tokens, locals);
        tokens = ret.1;
        locals = ret.2;

        node = new_node(nodekind, Some(node), Some(ret.0));
    }

    return (node, tokens, locals);
}

// unary = ("+" | "-")? primary
pub fn unary(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let mut tokens = tokens;
    let mut locals = locals;
    let token = tokens.front().unwrap();

    if token.kind == TokenKind::Reserved {
        if token.get_string() == "+" {
            tokens.pop_front();
            return primary(tokens, locals);
        }
        else if token.get_string() == "-" {
            tokens.pop_front();
            let _lhs = new_num_node(0);

            let ret = primary(tokens, locals);
            tokens = ret.1;
            locals = ret.2;

            let node = new_node(NodeKind::Sub, Some(_lhs), Some(ret.0));
            return (node, tokens, locals);
        }
    }

    return primary(tokens, locals);
}

// primary = num | ident | "(" expr ")"
pub fn primary(tokens: VecDeque<Token>, locals: Vec<LVar>) -> (Node, VecDeque<Token>, Vec<LVar>) {
    let mut tokens = tokens;
    let mut locals = locals;
    let token = tokens.pop_front().unwrap();

    if token.kind == TokenKind::Reserved {
        if token.get_string() == "(" {
            let (node, mut tokens, locals) = expr(tokens, locals);
            tokens.pop_front();
            return (node, tokens, locals);
        }
        panic!("not support operation");
    }

    if token.kind == TokenKind::Ident {
        let len = token.len;
        let name = token.get_string().to_string();
        let lvar = find_lvar(token, &locals);

        let offset = match lvar {
            Some(v) => v.offset,
            None => {
                let current_lvar = locals.last();
                let offset = match current_lvar {
                    Some(v) => v.offset + 8,
                    None => 8
                };
                let new_lvar = LVar {
                    name: name,
                    len: len,
                    offset: offset
                };
                locals.push(new_lvar);
                offset
            }
        };

        let node = Node {
            kind: NodeKind::LVar,
            lhs: Box::new(None),
            rhs: Box::new(None),
            val: 0,
            offset: offset as isize
        };

        return (node, tokens, locals);
    }
    
    let node = new_num_node(token.val.unwrap() as isize);
    return (node, tokens, locals);
}