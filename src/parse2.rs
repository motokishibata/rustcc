use crate::TokenType;

pub fn parse(tokens: &Vec<TokenType>) -> NodeType {
    let mut parser = Parser::new(tokens);
    parser.program()
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Num(i32),       // 数値
    Ident(String),  // 識別子
    Plus,
    Minus,
    Mul,
    Div,
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
    LVar(LVar),
    Unary(Box<NodeType>, Box<NodeType>),
    Multi(Box<NodeType>, Vec<(NodeType, NodeType)>),
    Add(Box<NodeType>, Vec<(NodeType, NodeType)>),
    Relational(Box<NodeType>, Vec<(NodeType, NodeType)>),
    Equality(Box<NodeType>, Vec<(NodeType, NodeType)>),
    Assign(Box<NodeType>, Option<Box<NodeType>>),
    Expr(Box<NodeType>),
    Stmt(Box<NodeType>),
    Return(Box<NodeType>),
    Program(Vec<NodeType>),        // top node
}

#[derive(Debug, Clone)]
pub struct LVar {
    pub name: String,
    pub len: i32,
    pub offset: i32,
}

pub struct Parser<'a> {
    tokens: &'a Vec<TokenType>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<TokenType>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    fn expect(&mut self, ty: TokenType) {
        if let Some(t) = self.tokens.get(self.pos) {
            if *t == ty {
                self.pos += 1;
                return;
            }
        }
        panic!("not expect charcter");
    }

    fn consume(&mut self, ty: TokenType) -> bool {
        if let Some(t) = self.tokens.get(self.pos) {
            if *t == ty {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    // program = stmt*
    fn program(&mut self) -> NodeType {
        let mut stmts = vec![];
        while self.tokens.len() != self.pos {
            stmts.push(self.stmt());
        }
        NodeType::Program(stmts)
    }

    // stmt =  expr ";" |
    //         "return" expr ";"
    fn stmt(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];

        match t {
            TokenType::Return => {
                self.pos += 1;
                let expr = self.expr();
                self.expect(TokenType::Semicolon);
                NodeType::Return(Box::new(expr))
            },
            _ => {
                let expr = self.expr();
                self.expect(TokenType::Semicolon);
                expr
            }
        }
    }

    // expr = assign
    fn expr(&mut self) -> NodeType {
        let assign = self.assign();
        NodeType::Expr(Box::new(assign))
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> NodeType {
        let eq = self.equality();
        let mut assign = None;
        if self.consume(TokenType::Assign) {
            assign = Some(Box::new(self.assign()));
        }
        NodeType::Assign(Box::new(eq), assign)
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> NodeType {
        let r1 = self.relational();
        let mut r2 = vec![];
        loop {
            let eq;
            if self.consume(TokenType::Eq) {
                eq = NodeType::Eq; 
            } else if self.consume(TokenType::Ne) {
                eq = NodeType::Ne;
            } else {
                break;
            }
            r2.push((eq, self.relational()));
        }
        NodeType::Equality(Box::new(r1), r2)
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> NodeType {
        let add1 = self.add();
        let mut add2 = vec![];
        loop {
            let op;
            if self.consume(TokenType::Lt) {
                op = NodeType::Lt;
            } else if self.consume(TokenType::Le) {
                op = NodeType::Le;
            } else if self.consume(TokenType::Gt) {
                op = NodeType::Gt;
            } else if self.consume(TokenType::Ge) {
                op = NodeType::Ge;
            } else {
                break;
            }
            add2.push((op, self.add()));
        }
        NodeType::Relational(Box::new(add1), add2)
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> NodeType {
        let mul1 = self.mul();
        let mut mul2 = vec![];
        loop {
            let op;
            if self.consume(TokenType::Plus) {
                op = NodeType::Plus;
            } else if self.consume(TokenType::Minus) {
                op = NodeType::Minus;
            } else {
                break;
            }
            mul2.push((op, self.mul()));
        }
        NodeType::Add(Box::new(mul1), mul2)
    }

    // mul = unary ("*" unary | "/" unary)*
    fn mul(&mut self) -> NodeType {
        let u1 = self.unary();
        let mut u2 = vec![];
        loop {
            let op;
            if self.consume(TokenType::Mul) {
                op = NodeType::Mul;
            } else if self.consume(TokenType::Div) {
                op = NodeType::Div;
            } else {
                break;
            }
            u2.push((op, self.unary()));
        }
        NodeType::Multi(Box::new(u1), u2)
    }

    // unary = ("+" | "-")? primary
    fn unary(&mut self) -> NodeType {
        let ope;
        if self.consume(TokenType::Minus) {
            ope = NodeType::Minus;
        } else {
            ope = NodeType::Plus;
        }
        NodeType::Unary(Box::new(ope), Box::new(self.primary()))
    }

    // primary = num | ident | "(" expr ")"
    fn primary(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];
        match t {
            TokenType::Num(val) => {
                self.pos += 1;
                NodeType::Num(*val)
            },
            TokenType::Ident(ident) => {
                self.pos += 1;
                NodeType::Ident(ident.into())
            },
            _ => {
                self.expect(TokenType::LeftParen);
                let nt = self.expr();
                self.expect(TokenType::RightParen);
                nt
            }
        }
    }
}